# CAPCF - Creative-Arts Provenance-Cognitive Framework

This project implements a modular software system based on the Creative-Arts Provenance-Cognitive Framework (CAPCF), consisting of five layers:

1. **Provenance Layer (PL)**: Append-only event logs, cryptographic signatures, artifact registry, versioned lineage tracking.
2. **Cognitive-Digital Interface Layer (CDI)**: Intention → representation pipelines, transformation tracking, feedback loop monitoring, cognitive lane protection mechanisms.
3. **Governance & Consent Layer (GL)**: Rights and licensing engine, consent envelopes, attribution resolver, policy enforcement.
4. **Intrusion Surface & Boundary Layer (IL)**: Stylistic mimicry detection, identity integrity checks, narrative boundary enforcement, unauthorized derivative detection.
5. **Cross-Domain Integration Layer (CDIL)**: Ethics, provenance, identity harmonization, interoperability schemas, system-wide consistency checks.

## Architecture

- Modular microservices architecture with clean separation of concerns.
- Strong typing and error handling.
- Languages: Rust for secure backend services (PL, GL, IL, CDIL), Go for CDI, Python for ML-based intrusion detection.

## Layers

- `provenance_layer/`: Rust service for provenance tracking.
- `cognitive_digital_interface_layer/`: Go service for cognitive interfaces.
- `governance_consent_layer/`: Rust service for governance and consent.
- `intrusion_surface_boundary_layer/`: Rust + Python for intrusion detection.
- `cross_domain_integration_layer/`: Rust service for integration.
- `shared/`: Common schemas and interfaces.

## Documentation

- [SPECIFICATION.md](SPECIFICATION.md): Detailed mechanisms and formal specification for each layer.
- [REQUIREMENTS.md](REQUIREMENTS.md): Formal requirements derived from the framework invariants.

## Building and Running

Each layer is a separate service. Refer to each layer's README for build instructions.

## Security and Provenance

All components prioritize security, provenance integrity, and agency preservation.