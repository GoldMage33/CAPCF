# CAPCF Formal Requirements

This document derives formal requirements (R1, R2, …) from the invariants specified in the CAPCF framework. Requirements are categorized by layer and include functional, security, and safety properties.

## Provenance Layer (PL) Requirements

**R1 (Signature Validity):** For every provenance event \( e \in \mathcal{E} \), the digital signature must be verifiable: \( Ver(actor_e, H(actor_e, in_e, op_e, out_e, ctx_e, id_e, t_e), sig_e) = true \).

**R2 (Append-Only Log):** The set of events must be non-decreasing over time: \( \mathcal{E}(t_2) \supseteq \mathcal{E}(t_1) \) for \( t_2 > t_1 \).

**R3 (DAG Property):** The provenance graph \( \mathcal{G}_P \) must remain acyclic.

**R4 (Block Integrity):** For tamper-evidence, each block hash must depend on the previous: \( h_{b_i} = H(events_{b_i} \,\|\, h_{b_{i-1}}) \) for \( i > 1 \), ensuring any alteration changes subsequent hashes.

**R5 (Event Creation):** The system must support creating events with all required fields and cryptographic signing.

**R6 (Lineage Queries):** The system must provide backward and forward lineage queries for any artifact \( a \in \mathcal{A} \).

## Cognitive-Digital Interface Layer (CDI) Requirements

**R7 (Intention Immutability):** Within a session \( \sigma \), intentions \( I_\sigma \) must only change via explicit user update operations.

**R8 (No Hidden Operations):** Any change in editor state must correspond to a recorded action \( a \in Act \).

**R9 (Lane Protection Precedence):** If \( protectLane(\sigma, a) = BLOCK \), the operation must not be committed to the Provenance Layer.

**R10 (Session Creation):** The system must allow starting sessions with user-specified intentions and constraints.

**R11 (Action Recording):** Every user operation must be recorded as an action with input/output states.

**R12 (Drift Detection):** The system must compute drift scores between intentions and generated artifacts.

## Governance and Consent Layer (GL) Requirements

**R13 (Consent Precedence):** If consent explicitly forbids a use, the decision must be DENY: \( checkConsent(Env, u, ctx) = DENY \Rightarrow decide(State_{GL}, u, ctx) = DENY \).

**R14 (No Silent Override):** Any ALLOW decision must be justified by an applicable policy or envelope.

**R15 (Traceability):** Every governed action must have a corresponding logged decision in \( Log_{GL} \).

**R16 (Policy Evaluation):** The system must evaluate policies against context and resolve conflicts using precedence rules.

**R17 (Consent Acquisition):** When permissions are needed, the system must prompt for user consent and log the decision.

**R18 (Attribution Resolution):** Using provenance chains, the system must compute required attributions for artifacts.

**R19 (License Propagation):** When combining artifacts, the system must compute and enforce resulting license constraints.

## Intrusion Surface and Boundary Layer (IL) Requirements

**R20 (Intrusion Recording):** If any detector (style, identity, narrative) returns true, an incident \( inc \in Inc \) must be created with status OPEN.

**R21 (Mitigation Binding):** If \( mitigate(inc) = BLOCK \), the suspect operation must not be committed to PL or exposed externally.

**R22 (Style Mimicry Detection):** The system must compute style embeddings and detect similarities exceeding thresholds without legitimate provenance links.

**R23 (Identity Integrity Checks):** The system must detect unauthorized use of identity traits and cross-check with consent.

**R24 (Narrative Boundary Enforcement):** The system must monitor thematic content and block outputs violating user-declared boundaries.

**R25 (Provenance-Based Anomaly Detection):** Using the provenance graph, the system must identify artifacts without plausible chains that match protected styles.

**R26 (Response Actions):** The system must support warnings, blocks, flagging, and escalation based on incident severity.

## Cross-Domain Integration Layer (CDIL) Requirements

**R27 (Agency Preservation):** For artifacts attributed to a user, no decisions may contradict session intentions beyond drift thresholds without explicit confirmation.

**R28 (Provenance Integrity):** Every public artifact must have a non-empty backward lineage: \( \forall a \in \mathcal{A}_{public} : lineage^{-}(a) \neq \emptyset \).

**R29 (Consent Supremacy):** Governed operations violating consent must be denied globally.

**R30 (Intrusion Non-Ignorability):** Critical incidents must prevent public exposure of suspect artifacts.

**R31 (Global Decision Function):** The system must compose checks from all layers to decide on operations.

**R32 (Consistency Checks):** The system must periodically verify no artifacts lack provenance, no consent violations, and no unresolved critical incidents.

**R33 (Ethical Guardrails):** Higher-order rules must override local optimizations, enforcing system-wide ethical constraints.

**R34 (Interoperability Mapping):** Internal structures must be mappable to external standards (JSON-LD, regulatory schemas) without losing guarantees.

**R35 (Profile Configuration):** Operational profiles must adjust thresholds and policies while respecting core invariants.

## System-Level Requirements

**R36 (State Machine Evolution):** The system must evolve as a state machine \( State(t+1) = \mathcal{T}(State(t), Input(t)) \), with \( \mathcal{T} \) composed of layer functions.

**R37 (Safety Property):** At all times, the global state must satisfy \( GInv \): \( \forall t: State(t) \models GInv \).

**R38 (Cross-Layer Reasoning):** For any operation, the system must consult provenance, governance, intrusion, and cognitive context.

**R39 (Invariant Enforcement):** All layer-specific invariants must be maintained, and violations must trigger corrective actions.

**R40 (Auditability):** All decisions, events, and state changes must be logged for audit purposes.

## Derived Functional Requirements

**R41 (API Endpoints):** Each layer must expose APIs for its core functions (e.g., event logging in PL, session management in CDI).

**R42 (Data Persistence):** Artifacts, events, sessions, and incidents must be persistently stored with integrity guarantees.

**R43 (Cryptographic Operations):** The system must support hashing, signing, and verification using secure algorithms.

**R44 (User Interfaces):** Users must be able to specify intentions, review decisions, and consent to operations.

**R45 (Integration Points):** Layers must communicate via defined interfaces (e.g., events from CDI to PL).

## Security Requirements

**R46 (Tamper Resistance):** Provenance logs and blocks must be tamper-evident.

**R47 (Access Control):** Operations must respect user permissions and consent envelopes.

**R48 (Intrusion Detection):** Continuous monitoring for stylistic, identity, and narrative intrusions.

**R49 (Privacy Preservation):** User data and intentions must be protected and not leaked.

**R50 (Agency Protection):** Automated systems must not override user intentions without consent.

## Performance and Scalability Requirements

**R51 (Query Efficiency):** Lineage and similarity queries must complete within acceptable time bounds.

**R52 (Concurrent Operations):** The system must handle multiple simultaneous sessions and operations.

**R53 (Storage Scalability):** Event logs and graphs must scale with artifact volume.

**R54 (Real-Time Monitoring):** Intrusion detection and drift monitoring must operate in near real-time.

## Testing and Validation Requirements

**R55 (Invariant Verification):** Automated tests must check all invariants after state changes.

**R56 (Formal Model Validation):** The system behavior must align with the mathematical model.

**R57 (Security Audits):** Regular audits of cryptographic operations and access controls.

**R58 (User Acceptance Testing):** End-to-end scenarios must validate agency preservation and ethical constraints.