use crate::controllers::Request;
use crate::error::{HarnessError, HarnessErrorType, HarnessResult};
use crate::soft_assert_eq;
use crate::{HarnessAgent, State};
use actix_web::{get, post, web, Responder};
use aries_vcx_agent::aries_vcx::agency_client::agency_client::AgencyClient;
use aries_vcx_agent::aries_vcx::handlers::connection::connection::Connection;
use aries_vcx_agent::aries_vcx::messages::a2a::A2AMessage;
use aries_vcx_agent::aries_vcx::messages::issuance::credential_offer::{
    CredentialOffer as VcxCredentialOffer, OfferInfo,
};
use aries_vcx_agent::aries_vcx::messages::issuance::credential_proposal::CredentialProposal as VcxCredentialProposal;
use aries_vcx_agent::aries_vcx::messages::issuance::credential_proposal::CredentialProposalData;
use aries_vcx_agent::aries_vcx::messages::issuance::CredentialPreviewData;
use aries_vcx_agent::aries_vcx::messages::mime_type::MimeType;
use aries_vcx_agent::aries_vcx::protocols::issuance::holder::state_machine::HolderState;
use aries_vcx_agent::aries_vcx::protocols::issuance::issuer::state_machine::IssuerState;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CredentialOffer {
    cred_def_id: String,
    credential_preview: CredentialPreviewData,
    connection_id: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct CredentialProposal {
    schema_issuer_did: String,
    issuer_did: String,
    schema_name: String,
    cred_def_id: String,
    schema_version: String,
    credential_proposal: CredentialPreviewData,
    connection_id: String,
    schema_id: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Credential {
    credential_preview: CredentialPreviewData,
    #[serde(default)]
    comment: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CredentialId {
    credential_id: String,
}

fn to_backchannel_state_issuer(state: IssuerState) -> State {
    match state {
        IssuerState::Initial => State::Initial,
        IssuerState::ProposalReceived | IssuerState::OfferSet => State::ProposalReceived,
        IssuerState::OfferSent => State::OfferSent,
        IssuerState::RequestReceived => State::RequestReceived,
        IssuerState::CredentialSent => State::CredentialSent,
        IssuerState::Finished => State::Done,
        IssuerState::Failed => State::Failure,
    }
}

fn to_backchannel_state_holder(state: HolderState) -> State {
    match state {
        HolderState::Initial => State::Initial,
        HolderState::ProposalSent => State::ProposalSent,
        HolderState::OfferReceived => State::OfferReceived,
        HolderState::RequestSent => State::RequestSent,
        HolderState::Finished => State::Done,
        HolderState::Failed => State::Failure,
    }
}

async fn download_tails_file(
    tails_base_url: &str,
    rev_reg_id: &str,
    tails_hash: &str,
) -> HarnessResult<()> {
    let url = match tails_base_url.to_string().matches('/').count() {
        0 => format!("{}/{}", tails_base_url, rev_reg_id),
        1.. => tails_base_url.to_string(),
        _ => {
            return Err(HarnessError::from_msg(
                HarnessErrorType::InternalServerError,
                "Negative count",
            ))
        }
    };
    let client = reqwest::Client::new();
    let tails_folder_path = std::env::current_dir()
        .expect("Failed to obtain the current directory path")
        .join("resource")
        .join("tails");
    std::fs::create_dir_all(&tails_folder_path).map_err(|_| {
        HarnessError::from_msg(
            HarnessErrorType::InternalServerError,
            "Failed to create tails folder",
        )
    })?;
    let tails_file_path = tails_folder_path
        .join(tails_hash)
        .to_str()
        .ok_or(HarnessError::from_msg(
            HarnessErrorType::InternalServerError,
            "Failed to convert tails hash to str",
        ))?
        .to_string();
    let res = client.get(&url).send().await?;
    soft_assert_eq!(res.status(), reqwest::StatusCode::OK);
    std::fs::write(tails_file_path, res.bytes().await?)?;
    Ok(())
}

async fn get_proposal(
    connection: &Connection,
    agency_client: &AgencyClient,
) -> HarnessResult<VcxCredentialProposal> {
    let mut proposals = Vec::<VcxCredentialProposal>::new();
    for (uid, message) in connection.get_messages(agency_client).await?.into_iter() {
        match message {
            A2AMessage::CredentialProposal(proposal) => {
                connection
                    .update_message_status(&uid, agency_client)
                    .await
                    .ok();
                proposals.push(proposal);
            }
            _ => {}
        }
    }
    soft_assert_eq!(proposals.len(), 1);
    proposals.pop().ok_or(HarnessError::from_msg(
        HarnessErrorType::InternalServerError,
        "Did not obtain presentation request message",
    ))
}

async fn get_offer(
    connection: &Connection,
    agency_client: &AgencyClient,
    thread_id: &str,
) -> HarnessResult<VcxCredentialOffer> {
    let mut offers = Vec::<VcxCredentialOffer>::new();
    for (uid, message) in connection.get_messages(agency_client).await?.into_iter() {
        match message {
            A2AMessage::CredentialOffer(offer) if offer.get_thread_id() == *thread_id => {
                connection
                    .update_message_status(&uid, agency_client)
                    .await
                    .ok();
                offers.push(offer);
            }
            _ => {}
        }
    }
    soft_assert_eq!(offers.len(), 1);
    offers.pop().ok_or(HarnessError::from_msg(
        HarnessErrorType::InternalServerError,
        "Did not obtain presentation request message",
    ))
}

impl HarnessAgent {
    pub async fn send_credential_proposal(
        &mut self,
        cred_proposal: &CredentialProposal,
    ) -> HarnessResult<String> {
        let mut proposal_data = CredentialProposalData::create()
            .set_schema_id(cred_proposal.schema_id.clone())
            .set_cred_def_id(cred_proposal.cred_def_id.clone());
        for attr in cred_proposal
            .credential_proposal
            .attributes
            .clone()
            .into_iter()
        {
            proposal_data =
                proposal_data.add_credential_preview_data(&attr.name, &attr.value, MimeType::Plain);
        }
        let id = self
            .aries_agent
            .holder()
            .send_credential_proposal(&cred_proposal.connection_id, proposal_data)
            .await?;
        let state = to_backchannel_state_holder(self.aries_agent.holder().get_state(&id)?);
        let proposals_len = self
            .aries_agent
            .connections()
            .get_credential_proposals(&cred_proposal.connection_id)
            .await?
            .len();
        Ok(json!({ "state": state, "thread_id": id }).to_string())
    }

    pub async fn send_credential_request(&mut self, id: &str) -> HarnessResult<String> {
        self.aries_agent
            .holder()
            .send_credential_request(Some(id), None)
            .await?;
        let state = to_backchannel_state_holder(self.aries_agent.holder().get_state(id)?);
        Ok(json!({ "state": state, "thread_id": id }).to_string())
    }

    pub async fn send_credential_offer(
        &self,
        cred_offer: &CredentialOffer,
        id: &str,
    ) -> HarnessResult<String> {
        let connection_id = if cred_offer.connection_id.is_empty() {
            None
        } else {
            Some(cred_offer.connection_id.as_str())
        };
        let (tails_file, rev_reg_id) = if !cred_offer.cred_def_id.is_empty() {
            let rev_reg_id = self
                .aries_agent
                .rev_regs()
                .find_by_cred_def_id(&cred_offer.cred_def_id)?
                .pop()
                .unwrap();
            (
                Some(self.aries_agent.rev_regs().get_tails_dir(&rev_reg_id)?),
                Some(rev_reg_id),
            )
        } else {
            (None, None)
        };
        let (offer_info, id) = if id.is_empty() {
            let credential_preview =
                serde_json::to_string(&cred_offer.credential_preview.attributes)?;
            (
                OfferInfo {
                    credential_json: credential_preview,
                    cred_def_id: cred_offer.cred_def_id.clone(),
                    rev_reg_id,
                    tails_file,
                },
                None,
            )
        } else {
            let connection_id = self.last_connection_id.as_ref().unwrap();
            let proposals = self
                .aries_agent
                .connections()
                .get_credential_proposals(connection_id)
                .await?;
            soft_assert_eq!(proposals.len(), 1);
            let proposal = proposals.last().unwrap();
            self.aries_agent
                .issuer()
                .accept_proposal(connection_id, proposal)
                .await?;
            (
                OfferInfo {
                    credential_json: proposal.credential_proposal.to_string().unwrap(),
                    cred_def_id: proposal.cred_def_id.clone(),
                    rev_reg_id,
                    tails_file,
                },
                Some(id),
            )
        };
        let id = self
            .aries_agent
            .issuer()
            .send_credential_offer(id, connection_id, offer_info)
            .await?;
        let state = to_backchannel_state_issuer(self.aries_agent.issuer().get_state(&id)?);
        Ok(json!({ "state": state, "thread_id": id }).to_string())
    }

    pub async fn issue_credetial(
        &self,
        id: &str,
        _credential: &Credential,
    ) -> HarnessResult<String> {
        self.aries_agent.issuer().update_state(id).await?;
        self.aries_agent.issuer().send_credential(id).await?;
        let state = to_backchannel_state_issuer(self.aries_agent.issuer().get_state(id)?);
        Ok(json!({ "state": state }).to_string())
    }

    pub async fn store_credential(&self, id: &str) -> HarnessResult<String> {
        // To send ack if necessary
        let state = self.aries_agent.holder().update_state(id).await?;

        if self.aries_agent.holder().is_revokable(id).await? {
            let rev_reg_id = self.aries_agent.holder().get_rev_reg_id(id).await?;
            let tails_hash = self.aries_agent.holder().get_tails_hash(id).await?;
            let tails_location = self.aries_agent.holder().get_tails_location(id).await?;
            download_tails_file(&tails_location, &rev_reg_id, &tails_hash).await?;
        };
        Ok(json!({ "state": to_backchannel_state_holder(state), "credential_id": id }).to_string())
    }

    pub async fn get_issuer_state(&mut self, id: &str) -> HarnessResult<String> {
        let state = if self.aries_agent.issuer().exists_by_id(&id) {
            to_backchannel_state_issuer(self.aries_agent.issuer().update_state(&id).await?)
        } else if self.aries_agent.holder().exists_by_id(&id) {
            to_backchannel_state_holder(self.aries_agent.holder().update_state(&id).await?)
        } else if let Some(connection_id) = &self.last_connection_id {
            let mut offers = self
                .aries_agent
                .connections()
                .get_credential_offers(connection_id)
                .await?;
            soft_assert_eq!(offers.len(), 1);
            let id = self
                .aries_agent
                .holder()
                .create_from_offer(connection_id, offers.pop().unwrap())?;
            to_backchannel_state_holder(self.aries_agent.holder().get_state(&id)?)
        } else {
            return Err(HarnessError::from_kind(HarnessErrorType::NotFoundError));
        };
        Ok(json!({ "state": state }).to_string())
    }

    pub async fn get_credential(&mut self, id: &str) -> HarnessResult<String> {
        Ok(json!({ "referent": id }).to_string())
    }
}

#[post("/send-proposal")]
pub async fn send_credential_proposal(
    req: web::Json<Request<CredentialProposal>>,
    agent: web::Data<Mutex<HarnessAgent>>,
) -> impl Responder {
    agent
        .lock()
        .unwrap()
        .send_credential_proposal(&req.data)
        .await
}

#[post("/send-offer")]
pub async fn send_credential_offer(
    req: web::Json<Request<CredentialOffer>>,
    agent: web::Data<Mutex<HarnessAgent>>,
) -> impl Responder {
    agent
        .lock()
        .unwrap()
        .send_credential_offer(&req.data, &req.id)
        .await
}

#[post("/send-request")]
pub async fn send_credential_request(
    req: web::Json<Request<String>>,
    agent: web::Data<Mutex<HarnessAgent>>,
) -> impl Responder {
    agent.lock().unwrap().send_credential_request(&req.id).await
}

#[get("/{issuer_id}")]
pub async fn get_issuer_state(
    agent: web::Data<Mutex<HarnessAgent>>,
    path: web::Path<String>,
) -> impl Responder {
    agent
        .lock()
        .unwrap()
        .get_issuer_state(&path.into_inner())
        .await
}

#[post("/issue")]
pub async fn issue_credential(
    req: web::Json<Request<Credential>>,
    agent: web::Data<Mutex<HarnessAgent>>,
) -> impl Responder {
    agent
        .lock()
        .unwrap()
        .issue_credetial(&req.id, &req.data)
        .await
}

#[post("/store")]
pub async fn store_credential(
    req: web::Json<Request<CredentialId>>,
    agent: web::Data<Mutex<HarnessAgent>>,
) -> impl Responder {
    agent.lock().unwrap().store_credential(&req.id).await
}

#[get("/{cred_id}")]
pub async fn get_credential(
    agent: web::Data<Mutex<HarnessAgent>>,
    path: web::Path<String>,
) -> impl Responder {
    agent
        .lock()
        .unwrap()
        .get_credential(&path.into_inner())
        .await
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/command/issue-credential")
            .service(send_credential_proposal)
            .service(send_credential_offer)
            .service(get_issuer_state)
            .service(send_credential_request)
            .service(issue_credential)
            .service(store_credential),
    )
    .service(web::scope("/command/credential").service(get_credential));
}
