# Aries Framework JavaScript Interoperability

## Runsets with AFJ

| Runset | ACME<br>(Issuer) | Bob<br>(Holder) | Faber<br>(Verifier) | Mallory<br>(Holder) | Scope | Results | 
| ------ | :--------------: | :-------------: | :----------------: | :-----------------: | ----- | :-----: | 
| [acapy-afj](#runset-acapy-afj) | acapy-main<br>1.0.0-rc0 | javascript<br>0.3.0-alpha.6 | acapy-main<br>1.0.0-rc0 | acapy-main<br>1.0.0-rc0 | AIP 1.0 | [**38 / 39<br>97%**](https://allure.vonx.io/api/allure-docker-service/projects/acapy-b-javascript/reports/latest/index.html?redirect=false#behaviors) |
| [acapy-dotnet-javascript](#runset-acapy-dotnet-javascript) | acapy-main<br>1.0.0-rc0 | javascript<br>0.3.0-alpha.6 | dotnet<br> | acapy-main<br>1.0.0-rc0 | AIP 1.0 | [**12 / 12<br>100%**](https://allure.vonx.io/api/allure-docker-service/projects/acapy-b-javascript-f-dotnet/reports/latest/index.html?redirect=false#behaviors) |
| [afj-acapy](#runset-afj-acapy) | javascript<br>0.3.0-alpha.6 | acapy-main<br>1.0.0-rc0 | javascript<br>0.3.0-alpha.6 | javascript<br>0.3.0-alpha.6 | AIP 1.0 | [**27 / 28<br>96%**](https://allure.vonx.io/api/allure-docker-service/projects/javascript-b-acapy/reports/latest/index.html?redirect=false#behaviors) |
| [afj-dotnet](#runset-afj-dotnet) | javascript<br>0.3.0-alpha.6 | dotnet<br> | javascript<br>0.3.0-alpha.6 | javascript<br>0.3.0-alpha.6 | AIP 1.0 | [**12 / 12<br>100%**](https://allure.vonx.io/api/allure-docker-service/projects/javascript-b-dotnet/reports/latest/index.html?redirect=false#behaviors) |
| [afj-findy](#runset-afj-findy) | javascript<br>0.3.0-alpha.6 | findy<br>0.30.42 | javascript<br>0.3.0-alpha.6 | javascript<br>0.3.0-alpha.6 | AIP 1.0 | [**17 / 17<br>100%**](https://allure.vonx.io/api/allure-docker-service/projects/javascript-b-findy/reports/latest/index.html?redirect=false#behaviors) |
| [afj](#runset-afj) | javascript<br>0.3.0-alpha.6 | javascript<br>0.3.0-alpha.6 | javascript<br>0.3.0-alpha.6 | javascript<br>0.3.0-alpha.6 | AIP 1.0 | [**28 / 28<br>100%**](https://allure.vonx.io/api/allure-docker-service/projects/javascript/reports/latest/index.html?redirect=false#behaviors) |
| [ariesvcx-javascript](#runset-ariesvcx-javascript) | aries-vcx<br>1.0.0 | javascript<br>0.3.0-alpha.6 | aries-vcx<br>1.0.0 | aries-vcx<br>1.0.0 | AIP 1.0 | [**20 / 20<br>100%**](https://allure.vonx.io/api/allure-docker-service/projects/aries-vcx-b-javascript/reports/latest/index.html?redirect=false#behaviors) |
| [dotnet-javascript](#runset-dotnet-javascript) | dotnet<br> | javascript<br>0.3.0-alpha.6 | dotnet<br> | dotnet<br> | AIP 1.0 | [**12 / 12<br>100%**](https://allure.vonx.io/api/allure-docker-service/projects/dotnet-b-javascript/reports/latest/index.html?redirect=false#behaviors) |
| [findy-javascript-dotnet](#runset-findy-javascript-dotnet) | findy<br>0.30.42 | javascript<br>0.3.0-alpha.6 | dotnet<br> | findy<br>0.30.42 | AIP 1.0 | [**17 / 17<br>100%**](https://allure.vonx.io/api/allure-docker-service/projects/findy-b-javascript-f-dotnet/reports/latest/index.html?redirect=false#behaviors) |
| [findy-javascript](#runset-findy-javascript) | findy<br>0.30.42 | javascript<br>0.3.0-alpha.6 | findy<br>0.30.42 | findy<br>0.30.42 | AIP 1.0 | [**17 / 17<br>100%**](https://allure.vonx.io/api/allure-docker-service/projects/findy-b-javascript/reports/latest/index.html?redirect=false#behaviors) |
| [javascript-ariesvcx](#runset-javascript-ariesvcx) | javascript<br>0.3.0-alpha.6 | aries-vcx<br>1.0.0 | javascript<br>0.3.0-alpha.6 | javascript<br>0.3.0-alpha.6 | AIP 1.0 | [**18 / 18<br>100%**](https://allure.vonx.io/api/allure-docker-service/projects/javascript-b-aries-vcx/reports/latest/index.html?redirect=false#behaviors) |

## Runset Notes

### Runset **acapy-afj**

Runset Name: ACA-PY to AFJ

```tip
**Latest results: 38 out of 39 (97%)**


*Last run: Sat Oct 29 00:58:16 UTC 2022*
```

#### Current Runset Status

Most of the tests are running. The tests not passing are being investigated.

*Status Note Updated: 2021.03.18*

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/acapy-b-javascript/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/acapy-b-javascript/reports/latest)


### Runset **acapy-dotnet-javascript**

Runset Name: ACA-PY to AF-.NET to AFJ

```tip
**Latest results: 12 out of 12 (100%)**


*Last run: Sat Oct 29 01:10:12 UTC 2022*
```

#### Current Runset Status

All tests are working, except for three tests that include Faber in the test run as an issuer.
These tests are; T001-RFC0037@1.2, T001.2-RFC0037@1.2, T001.4-RFC0037@1.1 . Further investigation 
is required to determine the issue in these three tests.

*Status Note Updated: 2021.03.18*

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/acapy-b-javascript-f-dotnet/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/acapy-b-javascript-f-dotnet/reports/latest)


### Runset **afj-acapy**

Runset Name: AFJ to ACA-PY

```tip
**Latest results: 27 out of 28 (96%)**


*Last run: Sat Oct 29 01:39:26 UTC 2022*
```

#### Current Runset Status

All AIP10 tests are currently running.

*Status Note Updated: 2021.03.17*

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/javascript-b-acapy/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/javascript-b-acapy/reports/latest)


### Runset **afj-dotnet**

Runset Name: AFJ to AF-.NET

```tip
**Latest results: 12 out of 12 (100%)**


*Last run: Sat Oct 29 01:32:38 UTC 2022*
```

#### Current Runset Status

All of the tests being executed in this runset are passing.

*Status Note Updated: 2021.03.05*

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/javascript-b-dotnet/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/javascript-b-dotnet/reports/latest)


### Runset **afj-findy**

Runset Name: AFJ to findy

```tip
**Latest results: 17 out of 17 (100%)**


*Last run: Sat Oct 29 01:34:54 UTC 2022*
```

#### Current Runset Status

All of the tests being executed in this runset are failing. There is an issue with afj sending the connection
response, and throws an error processing inbound message.

*Status Note Updated: 2021.09.28*

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/javascript-b-findy/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/javascript-b-findy/reports/latest)


### Runset **afj**

Runset Name: AFJ to AFJ

```tip
**Latest results: 28 out of 28 (100%)**


*Last run: Sat Oct 29 01:51:28 UTC 2022*
```

#### Current Runset Status

All of the tests being executed in this runset are passing.

*Status Note Updated: 2021.03.05*

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/javascript/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/javascript/reports/latest)


### Runset **ariesvcx-javascript**

Runset Name: aries-vcx to javascript

```tip
**Latest results: 20 out of 20 (100%)**


*Last run: Sat Oct 29 02:26:13 UTC 2022*
```

#### Current Runset Status
```warning
No test status note is available for this runset. Please update: .github/workflows/test-harness-ariesvcx-javascript.yml.
```

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/aries-vcx-b-javascript/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/aries-vcx-b-javascript/reports/latest)


### Runset **dotnet-javascript**

Runset Name: AF-.NET to AFJ

```tip
**Latest results: 12 out of 12 (100%)**


*Last run: Sat Oct 29 02:27:44 UTC 2022*
```

#### Current Runset Status

More tests are failing than are passing when Aries Framework .NET is playing the issuer role. More investigation is needed.

*Status Note Updated: 2021.03.18*

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/dotnet-b-javascript/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/dotnet-b-javascript/reports/latest)


### Runset **findy-javascript-dotnet**

Runset Name: findy to AFJ to AF-.NET

```tip
**Latest results: 17 out of 17 (100%)**


*Last run: Sat Oct 29 02:49:32 UTC 2022*
```

#### Current Runset Status

All of the tests being executed in this runset are passing. 

*Status Note Updated: 2021.10.15*

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/findy-b-javascript-f-dotnet/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/findy-b-javascript-f-dotnet/reports/latest)


### Runset **findy-javascript**

Runset Name: findy to AFJ

```tip
**Latest results: 17 out of 17 (100%)**


*Last run: Sat Oct 29 02:46:07 UTC 2022*
```

#### Current Runset Status

All of the tests being executed in this runset are passing. 

*Status Note Updated: 2021.10.15*

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/findy-b-javascript/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/findy-b-javascript/reports/latest)


### Runset **javascript-ariesvcx**

Runset Name: javascript to aries-vcx

```tip
**Latest results: 18 out of 18 (100%)**


*Last run: Sat Oct 29 03:15:31 UTC 2022*
```

#### Current Runset Status
```warning
No test status note is available for this runset. Please update: .github/workflows/test-harness-javascript-ariesvcx.yml.
```

#### Runset Details

- [Results by executed Aries RFCs](https://allure.vonx.io/api/allure-docker-service/projects/javascript-b-aries-vcx/reports/latest/index.html?redirect=false#behaviors)
- [Test execution history](https://allure.vonx.io/allure-docker-service-ui/projects/javascript-b-aries-vcx/reports/latest)

Jump back to the [interoperability summary](./README.md).

