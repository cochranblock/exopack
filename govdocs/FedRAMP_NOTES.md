<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# FedRAMP Applicability Notes — exopack

*2026-03-27.*

## Deployment Model

exopack is an **on-premises development tool**. It is not a cloud service, SaaS product, or hosted application.

| Question | Answer |
|----------|--------|
| Is this a cloud service? | No |
| Is this SaaS? | No |
| Is this IaaS/PaaS? | No |
| Is this on-prem software? | Yes — runs on developer workstations |
| Does it process federal data? | No — processes test artifacts only |
| Does it store data in the cloud? | No — all data local to the filesystem |
| Does it transmit data externally? | No — HTTP requests to localhost only |

## Authorization Boundary

Not applicable. exopack runs entirely within a developer's local machine. There is no network boundary, no multi-tenant architecture, and no shared infrastructure.

## FedRAMP Authorization

FedRAMP authorization is **not required** for exopack because:
1. It is not a cloud service offering
2. It does not process, store, or transmit federal information
3. It operates entirely on-premises with no external network dependencies
4. It is a build-time tool, not a deployed service

## Relevance to FedRAMP-Authorized Systems

exopack could be used as a **build tool** within the CI/CD pipeline of a FedRAMP-authorized system. In that context:
- exopack itself would be within the developer environment boundary, not the production boundary
- Its SBOM (`govdocs/SBOM.md`) documents all dependencies for audit
- Its supply chain documentation (`govdocs/SUPPLY_CHAIN.md`) satisfies continuous monitoring requirements for build tooling
