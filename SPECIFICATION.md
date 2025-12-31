# CAPCF Framework Specification

## Provenance Layer Mechanisms

This layer is the memory of the system: it records what happened, when, and by whom in a way that can't be silently rewritten.

### Data Structures

- **Event log**: Each event is a record: e = (id, timestamp, actor, artifact_in, operation, artifact_out, context, signature)
  - Stored in an append-only log (e.g. event store, blockchain, or tamper-evident log).

- **Artifact graph**: Nodes: artifacts (files, images, models, versions). Edges: "derived from", "edited by", "combined with".
  - This is analogous to provenance graphs in PIDS: nodes are entities, edges are operations forming a causal history of transformations.

- **Authorship chain**: Each artifact has a chain of signatures: chain(A) = [sig_author, sig_editor1, ..., sig_system]

### Core Mechanisms

1. **Event capture**: Whenever a user or system performs an operation (create, edit, export, train, derive), a new event is emitted.
   - The event includes: hash of input artifact(s), hash of output artifact, operation type and parameters, actor identity, timestamp, digital signature over all of the above.

2. **Hashing and linking**: Artifacts are hashed (e.g. SHA-256). Events link hashes of input and output artifacts, forming a directed acyclic graph (DAG). Tampering with content changes the hash, which breaks the chain.

3. **Cryptographic sealing**: Events are batched into blocks or checkpoints. Each block includes: block_hash = H(events || prev_block_hash). Optionally anchored to an external chain (public blockchain) or hardware security module to strengthen tamper-evidence.

4. **Query and reconstruction**: Given an artifact, you can: trace back to its origin (backward traversal), trace forward to all derivatives (forward traversal), reconstruct the entire lineage graph.

5. **Provenance-based detection hooks**: The event stream and graph can feed intrusion detection, similar to provenance-based intrusion detection systems (PIDS) that analyze provenance graphs to detect anomalies and reconstruct attacks.

### What it feels like in use

- Every creative step is logged as a small, signed, linked event.
- The system can always answer: "Where did this come from?", "Who touched it?", "What exactly changed?".

## Cognitive-Digital Interface Layer Mechanisms

This layer is where intention enters the system and becomes a traceable sequence of transformations, rather than getting swallowed by black-box tools.

### Data Structures

- **Session model**: A "creative session": session = (session_id, user_id, start_time, tool_context, goals, constraints)

- **Action stream**: Fine-grained steps: edits, prompts, brush strokes, prompt updates, parameter changes.

- **Intention metadata**: Declared goals (e.g. "style: minimal", "no faces", "non-commercial"). Constraints representing what the user does not want.

### Core Mechanisms

1. **Intention capture**: Explicit: user sets goals and constraints in UI (e.g. "no training on this", "no reuse"). Implicit: the system infers session focus (e.g. subject, style) from actions, without overriding explicit statements.

2. **Transformation pipeline**: Every user operation becomes: (input_state, operation, parameters) → output_state. The pipeline is composable and observable: You can see the chain of operations (like a functional composition). Each step can be recorded in the provenance layer.

3. **Feedback loop monitoring**: The system observes: what suggestions or auto-completions it is giving, what the user accepts/rejects, divergence between initial intention and emerging artifact. It can detect "drift" where the system starts pulling the user into patterns that are not aligned with the declared intention.

4. **Cognitive lane protection**: Rules such as: "Do not auto-introduce motifs the user explicitly excluded.", "Do not override user's stylistic cues with system defaults unless explicitly requested.", "Flag when generated content strongly resembles training examples or other users' work." These rules act as guards preventing the system from quietly co-authoring in a way that erodes agency.

5. **Commit to provenance**: When a user commits/exports/saves, the CDI layer: summarizes the intention and key decisions, passes a structured record to the provenance layer as the semantic context of the events.

### Intuition

- This layer is about making sure the system is an instrument the user plays, not a ghost writer.
- Every nudge, suggestion, and transformation is logged and constrained so it can't silently hijack the creative path.

## Governance and Consent Layer Mechanisms

Here, we encode who is allowed to do what, when, and under which conditions — and we enforce it against provenance facts.

### Data Structures

- **Consent envelope**: For an artifact or session: envelope = (artifact_id, owner, allowed_uses, forbidden_uses, revocation_rules, expiry, signatures)

- **Policy rules**: Declarative rules like: "If artifact is tagged PERSONAL and not explicitly LICENSED_PUBLIC, disallow training and commercial reuse.", "Always preserve original author attribution in derivatives."

- **Rights graph**: Links between: people (creators, collaborators, licensees), artifacts, licenses, contracts.

### Core Mechanisms

1. **Policy evaluation**: For any requested action (e.g. "train on this dataset", "export under CC BY"), the system: collects applicable policies (global, user, artifact-specific), checks them against provenance facts and consent envelopes, returns ALLOW / DENY / REQUIRE-ESCALATION.

2. **Consent acquisition**: When the system needs permissions beyond existing envelopes: it generates a clear, human-readable explanation ("To do X, we need Y."), the user accepts or rejects; the decision is logged and signed, a new or updated consent envelope is created.

3. **Attribution resolution**: Uses the provenance graph and authorship chains to: compute who must be credited, in what order and form, whether an attribution condition is satisfied or violated.

4. **License propagation**: When artifacts are combined: the system computes the resulting license constraints (intersection / composition of terms), if incompatible, it blocks the operation or prompts for different inputs.

5. **Compliance logging**: All policy decisions (ALLOW/DENY) are themselves recorded as events: what was requested, what rule applied, who decided (user, system, admin), outcome.

### Effect

- The system can't "accidentally" train on something it shouldn't or generate a derivative that violates consent; the logic is explicit, inspectable, and backed by provenance facts.

## Intrusion Surface and Boundary Layer Mechanisms

This layer treats style, identity, narrative, and authorship as things that can be attacked (mimicked, hijacked, diluted) and thus need monitoring and defense.

### Data Structures

- **Embedding spaces**: Style embeddings (for visual or textual style). Identity embeddings (e.g. characteristic patterns of a creator's work). Narrative/topic embeddings (what the work is about).

- **Intrusion rules**: e.g. "Impersonation threshold for style similarity", "forbidden identity use", "narrative hijack patterns".

- **Incident records**: Each suspected intrusion: incident = (source_artifact, suspect_artifact, similarity_scores, rule_triggered, severity, status)

### Core Mechanisms

1. **Stylistic mimicry detection**: Compute embeddings of new artifacts. Compare against: the original creator's corpus, other protected styles. If similarity exceeds a threshold and there is no provenance link that legitimizes reuse, raise an alert.

2. **Identity integrity checks**: Detect use of names, signatures, watermarks, or recognizable personal traits. Cross-check with consent: is this person allowed to be represented here? was consent given for this type of use?

3. **Narrative boundary enforcement**: Track thematic content: If a user's work is consistently focused on A, but the system starts generating B in ways that conflict with their declared boundaries, that is a cognitive/narrative intrusion. For example: user declares "no political content"; system must detect and block politicized outputs.

4. **Provenance-based anomaly detection**: Use the provenance graph as a structured signal, similar to PIDS that build provenance graphs from system logs to detect malicious behavior. Example patterns: An artifact appears without any plausible provenance chain but strongly matches a protected style. A derivative claims attribution that the provenance graph doesn't support.

5. **Response actions**: Soft responses: warnings, explanation, logs to creator. Hard responses: block export, mark artifact as "contested", require additional consent or legal escalation.

### Conceptual Effect

- This layer is the immune system: it watches for stylistic, narrative, and identity-based attacks and refuses to treat every new artifact as innocent just because it exists.

## Cross-Domain Integration Layer Mechanisms

This is the orchestration and coherence layer: it keeps ethics, provenance, cognition, identity, and governance aligned.

### Data Structures

- **Unified state model**: A joined view over: provenance facts, current policies, active sessions, intrusion status, ethical constraints.

- **Semantic schemas**: JSON-LD or similar for artifacts, people, rights, and events, so all layers talk in consistent terms.

- **Constraint set**: Global invariants like: "No operation may violate explicit consent.", "No artifact may be detached from its provenance chain.", "No automated action may change authorship attribution."

### Core Mechanisms

1. **Cross-layer reasoning**: For a given operation, the CDIL: consults provenance (what is this, where from?), consults governance (what's allowed?), consults intrusion (any risks?), consults cognitive context (what did user intend?). It then allows, modifies, or denies the operation based on joint constraints.

2. **Ethical guardrails**: Encodes higher-order rules that override local optimizations: even if something is legal and technically possible, it may be ethically disallowed (e.g. manipulative uses, exploitative generation). These show up as system-wide "thou shalt nots".

3. **Interoperability and export**: Handles mapping internal structures to: external standards, regulatory reporting schemas, audit formats. Ensures that when data leaves the system, its ethics and provenance guarantees don't silently disappear.

4. **Consistency checks**: Periodically or on-demand: verify no artifacts exist without provenance, verify all uses match consent, verify no unresolved intrusion incidents for public artifacts.

5. **Configurable profiles**: Different operational modes: research sandbox, production constrained mode, high-sensitivity mode (e.g. for vulnerable creators). Each profile sets different thresholds and policies, but all still respect core invariants.

### Why This Matters

- Without this layer, you just have a pile of clever subsystems.
- With it, you have a coherent ethical machine: every decision is made in view of provenance, consent, cognition, and intrusion risk, not in isolation.

## Formal Specification

Below is a formal specification of the framework as if you were writing a system design / formal model section in a thesis or IEEE article. I'll use simple mathematical notation, sets, functions, and invariants, and keep it implementation-agnostic but precise.

### 0. Notation and Basic Sets

Let:

- **Actors and identities**
  - \( U \) — set of users (creators, collaborators, admins)
  - \( S \) — set of system components/services
  - \( A = U \cup S \) — set of all actors

- **Artifacts and sessions**
  - \( \mathcal{A} \) — set of artifacts (digital or physical representations)
  - \( \Sigma \) — set of creative sessions

- **Time**
  - \( T \) — totally ordered set of timestamps

- **Operations and policies**
  - \( Op \) — set of operation types (e.g. create, edit, combine, export, train, etc.)
  - \( Pol \) — set of policies
  - \( Inc \) — set of intrusion incidents

- **Identifiers and cryptography**
  - \( ID \) — set of unique identifiers
  - \( H : \{0,1\}^* \rightarrow \{0,1\}^n \) — cryptographic hash function
  - \( Sig : A \times \{0,1\}^* \rightarrow \{0,1\}^* \) — digital signature function
  - \( Ver : A \times \{0,1\}^* \times \{0,1\}^* \rightarrow \{true,false\} \) — signature verification

### 1. System Model

The system is a tuple:

\[\mathsf{CAPCF} = (PL, CDI, GL, IL, CDIL)\]

where each component \( X \in \{PL, CDI, GL, IL, CDIL\} \) is itself a structured tuple of state, functions, and invariants.

### 2. Provenance Layer \(PL\)

#### 2.1 Core Sets and Types

- \( \mathcal{A} \) — artifacts
- \( \mathcal{E} \) — set of provenance events
- \( \mathcal{G}_P = (\mathcal{A}, E_P) \) — provenance graph, where \( E_P \subseteq \mathcal{A} \times \mathcal{A} \)

Define an event as a tuple:

\[e \in \mathcal{E},\quad e = (id_e, t_e, actor_e, in_e, op_e, out_e, ctx_e, sig_e)\]

where:

- \( id_e \in ID \)
- \( t_e \in T \)
- \( actor_e \in A \)
- \( in_e \subseteq \mathcal{A} \) — input artifacts
- \( op_e \in Op \) — operation type
- \( out_e \subseteq \mathcal{A} \) — output artifacts
- \( ctx_e \in Ctx \) — context (metadata)
- \( sig_e \in \{0,1\}^* \) — digital signature

Each artifact \( a \in \mathcal{A} \) has:

\[a = (id_a, h_a, meta_a)\]

where \( h_a = H(content(a)) \).

#### 2.2 State of PL

\[State_{PL} = (\mathcal{E}, \mathcal{A}, \mathcal{G}_P, B)\]

where \( B \) is a sequence of blocks (for tamper-evidence):

\[B = [b_1, b_2, \ldots, b_n]\]

and each block \( b_i = (id_{b_i}, events_{b_i}, h_{b_i}) \) with:

\[h_{b_i} = H(events_{b_i} \,\|\, h_{b_{i-1}})\]

and \( h_{b_0} \) a fixed genesis value.

#### 2.3 Functions

**Event creation:**

\[createEvent : A \times 2^{\mathcal{A}} \times Op \times 2^{\mathcal{A}} \times Ctx \rightarrow \mathcal{E}\]

Given \( (actor, in, op, out, ctx) \), define:

1. Compute payload: \(payload = (actor, in, op, out, ctx)\)
2. Let \( id_e \in ID \) be freshly generated.
3. Let \( t_e \in T \) be current time.
4. Compute signature: \(sig_e = Sig(actor, H(payload \,\|\, id_e \,\|\, t_e))\)
5. Return: \(e = (id_e, t_e, actor, in, op, out, ctx, sig_e)\)

**Event append:**

\[appendEvent : State_{PL} \times \mathcal{E} \rightarrow State_{PL}\]

Adds event \( e \) to \( \mathcal{E} \), updates \( \mathcal{G}_P \) with edges:

\[\forall a_{in} \in in_e, \forall a_{out} \in out_e: (a_{in}, a_{out}) \in E_P\]

and includes \( e \) into the current block \( b_n \) or starts a new block per block policy.

**Lineage queries:**

- Backward: \(lineage^{-}(a) = \{ a^\prime \in \mathcal{A} \mid \exists \text{ path } a^\prime \rightarrow^* a \text{ in } \mathcal{G}_P \}\)
- Forward: \(lineage^{+}(a) = \{ a^\prime \in \mathcal{A} \mid \exists \text{ path } a \rightarrow^* a^\prime \text{ in } \mathcal{G}_P \}\)

#### 2.4 Invariants

1. Signature validity:

\[\forall e \in \mathcal{E} : Ver(actor_e, H(actor_e, in_e, op_e, out_e, ctx_e, id_e, t_e), sig_e) = true\]

2. Append-only log:

\[\mathcal{E}(t_2) \supseteq \mathcal{E}(t_1) \quad \text{for } t_2 > t_1\]

3. DAG property:

\[\mathcal{G}_P \text{ is acyclic}\]

4. Block integrity:

\[\forall i > 1 : h_{b_i} = H(events_{b_i} \,\|\, h_{b_{i-1}})\]

Any change in \( events_{b_i} \) or previous block hashes must change \( h_{b_i} \).

### 3. Cognitive-Digital Interface \(CDI\)

#### 3.1 Core Sets

- \( \Sigma \) — sessions
- \( Act \) — atomic actions (edits, prompts, parameter changes)
- \( I \) — intentions
- \( Cst \) — constraints

Define a session:

\[\sigma \in \Sigma, \quad \sigma = (id_\sigma, user_\sigma, t_{start}, ctx_\sigma, I_\sigma, Cst_\sigma)\]

Define an action:

\[a \in Act, \quad a = (id_a, \sigma_a, t_a, op_a, params_a, state\_in_a, state\_out_a)\]

where \( state\_in_a, state\_out_a \in St \) (space of editor/tool states).

#### 3.2 State of CDI

\[State_{CDI} = (\Sigma, Act, St, L)\]

where \( L \) is a log of feedback loop data (e.g. suggestions, accept/reject info).

#### 3.3 Functions

**Session creation:**

\[startSession : U \times Ctx \times I \times 2^{Cst} \rightarrow \Sigma\]

Returns new \( \sigma \) with fresh \( id_\sigma \), current time, and given intention and constraints.

**Action application:**

\[applyAction : St \times Op \times Params \rightarrow St\]

Given current state and operation, returns next state (deterministic or stochastic).

**Action record:**

\[recordAction : \Sigma \times Op \times Params \times St \rightarrow (Act, St)\]

Given session \( \sigma \), operation \( op \), params, and current state \( s \):

1. Let \( s^\prime = applyAction(s, op, params) \).
2. Create action \( a \) with: \( state\_in_a = s \), \( state\_out_a = s^\prime \)
3. Append \( a \) to \( Act \).
4. Return \( (a, s^\prime) \).

**Drift detection (intention vs output):**

\[drift : I \times Cst \times \mathcal{A} \rightarrow \mathbb{R}_{\geq 0}\]

Maps intention + constraints + current artifact to a drift score (0 = aligned, higher = misaligned).

**Lane protection decision:**

\[protectLane : \sigma \times \mathcal{A} \rightarrow Decision\]

Where \( Decision \in \{ALLOW, WARN, BLOCK, ADJUST\} \), based on \( drift(I_\sigma, Cst_\sigma, a) \) and configured thresholds.

#### 3.4 Invariants

1. Intention immutability within session (unless explicitly updated):

\[\forall \sigma \in \Sigma,\ \forall a \in Act, \ a.\sigma_a = \sigma \Rightarrow I_\sigma \text{ changes only via explicit update operation}\]

2. No hidden operations:

\[\forall \sigma, \forall t, \ s(t+\Delta t) \neq s(t) \Rightarrow \exists a \in Act: state\_out_a = s(t+\Delta t)\]

(in practice, up to granularity constraints).

3. Lane protection precedence:

\[If \ protectLane(\sigma, a) = BLOCK, \ the \ corresponding \ operation \ must \ not \ be \ committed \ to \ PL.\]

### 4. Governance and Consent Layer \(GL\)

#### 4.1 Core Sets

- \( Env \) — set of consent envelopes
- \( UseType \) — types of use (train, commercial, derivative, etc.)
- \( Outcome \in \{ALLOW, DENY, ESCALATE\} \)

A consent envelope:

\[env \in Env, \quad env = (artifact\_id, owner, allowed, forbidden, revocation, expiry, sig_{env})\]

where:

- \( allowed, forbidden \subseteq UseType \times Context \)
- \( revocation \) — rules for revoking consent
- \( expiry \in T \cup \{\infty\} \)

A policy:

\[pol \in Pol, \quad pol : Context \rightarrow Outcome\]

Context includes: artifact ids, actors, envs, provenance facts.

#### 4.2 State of GL

\[State_{GL} = (Env, Pol, Log_{GL})\]

#### 4.3 Functions

**Policy evaluation:**

\[evalPolicies : Pol \times Context \rightarrow Outcome\]

Aggregates all applicable policies and resolves conflicts using a defined precedence relation \( \prec \) (e.g. DENY > ALLOW, explicit > default).

**Consent check:**

\[checkConsent : Env \times UseType \times Context \rightarrow Outcome\]

Given envelopes, use type, and context, returns:

- ALLOW if use type is in allowed and not in forbidden and not expired or revoked.
- DENY if explicitly forbidden or expired/revoked.
- ESCALATE otherwise.

**Governed action decision:**

\[decide : State_{GL} \times UseType \times Context \rightarrow Outcome\]

Formally:

1. Let \( o_1 = evalPolicies(Pol, Context) \).
2. Let \( o_2 = checkConsent(Env, UseType, Context) \).
3. Combine \( o_1, o_2 \) using precedence rules to yield final \( o \in Outcome \).

**Decision logging:**

\[logDecision : Outcome \times Context \rightarrow Log_{GL}\]

Appends an entry containing the decision, context, time, and rule trace.

#### 4.4 Invariants

1. Consent precedence:

\[If \ checkConsent(Env, u, ctx) = DENY \Rightarrow decide(State_{GL}, u, ctx) = DENY\]

2. No silent override:

\[decide(State_{GL}, u, ctx) = ALLOW \Rightarrow \exists pol \in Pol \lor env \in Env \text{ that justifies ALLOW}\]

3. Traceability:

\[\forall \text{ governed action } g: \exists entry \in Log_{GL} \text{ describing } g\]

### 5. Intrusion Surface and Boundary Layer \(IL\)

#### 5.1 Core Sets and Functions

- \( Emb \) — embedding space (e.g. \( \mathbb{R}^d \))
- \( StyleEmb, IdEmb, NarrEmb \subseteq Emb \) — style, identity, narrative embeddings
- \( Sim : Emb \times Emb \rightarrow \mathbb{R}_{\geq 0} \) — similarity function (higher = more similar)
- \( Th_{style}, Th_{id}, Th_{narr} \) — thresholds

For each artifact \( a \in \mathcal{A} \):

\[style(a) \in StyleEmb,\quad id(a) \in IdEmb,\quad narr(a) \in NarrEmb\]

An intrusion incident:

\[inc \in Inc, \quad inc = (id_{inc}, source, suspect, type, scores, rule, severity, status)\]

where \( type \in \{\text{STYLE_MIMIC}, \text{ID_MISUSE}, \text{NARRATIVE_DRIFT}\} \), \( status \in \{OPEN, CLOSED\} \).

#### 5.2 State of IL

\[State_{IL} = (Inc, Th_{style}, Th_{id}, Th_{narr}, Rules_{IL})\]

#### 5.3 Functions

**Style intrusion detection:**

\[detectStyleIntrusion : \mathcal{A} \times \mathcal{A} \rightarrow \{true, false\}\]

\[detectStyleIntrusion(a_{ref}, a_{sus}) = \begin{cases} true & \text{if } Sim(style(a_{ref}), style(a_{sus})) > Th_{style} \land \neg LegitimateDerivative(a_{ref}, a_{sus}) \\ false & \text{otherwise} \end{cases}\]

where \( LegitimateDerivative \) is derived from provenance and consent.

**Identity intrusion detection:**

\[detectIdentityIntrusion(a_{ref}, a_{sus}) = \begin{cases} true & \text{if } Sim(id(a_{ref}), id(a_{sus})) > Th_{id} \land \neg AuthorizedIdentityUse(a_{ref}, a_{sus}) \\ false & \text{otherwise} \end{cases}\]

**Narrative intrusion or drift:**

\[detectNarrativeIntrusion(\sigma, a) = \begin{cases} true & \text{if } drift(I_\sigma, Cst_\sigma, a) > Th_{narr} \\ false & \text{otherwise} \end{cases}\]

**Incident creation:**

\[createIncident : Context \rightarrow Inc\]

Creates an \( inc \) with type and scores determined by which detector fired.

**Mitigation decision:**

\[mitigate : Inc \rightarrow Decision\]

Where \( Decision \in \{WARN, BLOCK, FLAG, ESCALATE\} \) based on severity and rules.

#### 5.4 Invariants

1. No unrecorded intrusions:

\[If \ any \ detector \ returns \ true: detectStyleIntrusion \lor detectIdentityIntrusion \lor detectNarrativeIntrusion \Rightarrow \exists inc \in Inc: status(inc) = OPEN\]

2. Mitigation binding:

\[If \ mitigate(inc) = BLOCK, \ the \ corresponding \ operation \ must \ not \ be \ committed \ to \ PL \ or \ exposed \ externally.\]

### 6. Cross-Domain Integration Layer \(CDIL\)

#### 6.1 Core Sets

- \( State_{PL}, State_{CDI}, State_{GL}, State_{IL} \) — as above
- \( GInv \) — set of global invariants (ethics, agency, provenance integrity, etc.)

A global context:

\[Ctx_{global} = (State_{PL}, State_{CDI}, State_{GL}, State_{IL}, External)\]

where \( External \) includes regulatory or platform-level info.

#### 6.2 State of CDIL

\[State_{CDIL} = (GInv, Profiles, Log_{CDIL})\]

where Profiles defines operational modes (e.g. STRICT, RESEARCH, STANDARD).

#### 6.3 Functions

**Global decision function:**

\[globalDecide : Ctx_{global} \times Operation \rightarrow Outcome\]

This composes:

1. Provenance checks (existence, lineage consistency).
2. Governance decision \( decide(State_{GL}, UseType, Context) \).
3. Intrusion checks \( detect* \).
4. Lane protection \( protectLane \).

and returns a final outcome \( Outcome \in \{ALLOW, DENY, WARN, ESCALATE\} \).

**Consistency checking:**

\[checkConsistency : Ctx_{global} \rightarrow \{true, false\}\]

Ensures:

- every public artifact has a provenance chain,
- no known consent violations,
- no OPEN critical incidents.

**Profile application:**

\[applyProfile : Profile \times GInv \rightarrow GInv^\prime\]

Modifies thresholds and secondary rules but must not violate core invariants.

#### 6.4 Global Invariants (Examples)

1. Agency preservation:

\[For \ any \ creative \ session \ and \ its \ final \ artifact \ a_f: If \ a_f \ is \ attributed \ to \ user \ u, \ then \ no \ logged \ decision \ contradicts \ I_\sigma \ in \ a \ way \ that \ exceeds \ configured \ drift \ thresholds \ without \ explicit \ confirmation \ from \ u.\]

2. Provenance integrity:

\[\forall a \in \mathcal{A}_{public} : lineage^{-}(a) \neq \emptyset\]

Public artifacts must have a traceable origin.

3. Consent supremacy:

\[\forall \text{ governed operations } op: \text{if } checkConsent(Env, useType(op), ctx) = DENY \Rightarrow globalDecide(Ctx_{global}, op) = DENY\]

4. Intrusion non-ignorability:

\[\forall inc \in Inc \text{ with severity } \geq s_{crit}: status(inc) \neq CLOSED \Rightarrow \text{no public exposure of suspect artifact}\]

### 7. System Evolution

The system runs as a state machine:

\[State(t+1) = \mathcal{T}(State(t), Input(t))\]

where:

\[State = (State_{PL}, State_{CDI}, State_{GL}, State_{IL}, State_{CDIL})\]

and \( \mathcal{T} \) is the transition function composed of:

- action recording (CDI),
- provenance event creation and append (PL),
- policy/consent decisions (GL),
- intrusion detection and mitigation (IL),
- global decision and invariant enforcement (CDIL).

**Safety property:**

\[\forall t: State(t) \models GInv\]

The system must always satisfy the global invariants.