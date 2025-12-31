# CAPCF API Schemas

This document defines API schemas for the CAPCF framework layers, corresponding to the formal functions and data structures. APIs use a combination of REST (for simplicity), gRPC (for typed interfaces), and GraphQL (for flexible queries). All APIs include authentication, authorization via GL, and logging to PL.

## Provenance Layer (PL) APIs

Base URL: `/api/pl`

### REST Endpoints

**POST /events**
- Create and append a new event.
- Request Body: `Event` (JSON)
- Response: `Event` with ID and signature
- Maps to: `createEvent` and `appendEvent`

**GET /events**
- Query events with filters.
- Query Params: `event_type`, `start_time`, `end_time`
- Response: Array of `Event`
- Maps to: `get_events`

**GET /artifacts/{id}/lineage**
- Get lineage for an artifact.
- Path Param: `id` (UUID)
- Query Param: `direction` (backward|forward)
- Response: `Lineage` object
- Maps to: `get_lineage`

**POST /artifacts**
- Register a new artifact.
- Request Body: `Artifact` (JSON)
- Response: `Artifact` with ID
- Maps to: `register_artifact`

### gRPC Service

```protobuf
service ProvenanceService {
  rpc CreateEvent(CreateEventRequest) returns (Event);
  rpc GetEvents(GetEventsRequest) returns (stream Event);
  rpc GetLineage(LineageRequest) returns (Lineage);
  rpc RegisterArtifact(Artifact) returns (Artifact);
}

message CreateEventRequest {
  string actor = 1;
  repeated string in_artifacts = 2;
  string operation = 3;
  repeated string out_artifacts = 4;
  string context = 5;
}

message GetEventsRequest {
  string event_type = 1;
  string start_time = 2;
  string end_time = 3;
}

message LineageRequest {
  string artifact_id = 1;
  string direction = 2; // "backward" or "forward"
}
```

### GraphQL Schema

```graphql
type Query {
  events(filter: EventFilter): [Event!]!
  lineage(artifactId: ID!, direction: Direction): Lineage
  artifacts: [Artifact!]!
}

type Mutation {
  createEvent(input: CreateEventInput!): Event!
  registerArtifact(input: ArtifactInput!): Artifact!
}

enum Direction {
  BACKWARD
  FORWARD
}

input EventFilter {
  eventType: String
  startTime: DateTime
  endTime: DateTime
}
```

## Cognitive-Digital Interface Layer (CDI) APIs

Base URL: `/api/cdi`

### REST Endpoints

**POST /sessions**
- Start a new session.
- Request Body: `{user_id, context, intentions, constraints}`
- Response: `Session`
- Maps to: `startSession`

**POST /sessions/{id}/actions**
- Record an action in a session.
- Path Param: `id` (session ID)
- Request Body: `{operation, params, state_in}`
- Response: `{action, state_out}`
- Maps to: `recordAction`

**GET /sessions/{id}/representations**
- Get representations for a session.
- Response: Array of `Representation`
- Maps to: `GetRepresentations`

**POST /drift**
- Compute drift score.
- Request Body: `{intentions, constraints, artifact}`
- Response: `{score: float}`
- Maps to: `drift`

### gRPC Service

```protobuf
service CognitiveService {
  rpc StartSession(StartSessionRequest) returns (Session);
  rpc RecordAction(RecordActionRequest) returns (ActionResponse);
  rpc GetRepresentations(SessionId) returns (stream Representation);
  rpc ComputeDrift(DriftRequest) returns (DriftResponse);
}

message StartSessionRequest {
  string user_id = 1;
  string context = 2;
  repeated string intentions = 3;
  repeated string constraints = 4;
}

message RecordActionRequest {
  string session_id = 1;
  string operation = 2;
  string params = 3;
  string state_in = 4;
}

message ActionResponse {
  Action action = 1;
  string state_out = 2;
}

message DriftRequest {
  repeated string intentions = 1;
  repeated string constraints = 2;
  string artifact = 3;
}

message DriftResponse {
  double score = 1;
}
```

### GraphQL Schema

```graphql
type Query {
  session(id: ID!): Session
  representations(sessionId: ID!): [Representation!]!
}

type Mutation {
  startSession(input: StartSessionInput!): Session!
  recordAction(sessionId: ID!, input: ActionInput!): Action!
  computeDrift(input: DriftInput!): Float!
}

input StartSessionInput {
  userId: ID!
  context: String
  intentions: [String!]
  constraints: [String!]
}

input ActionInput {
  operation: String!
  params: String
  stateIn: String
}
```

## Governance and Consent Layer (GL) APIs

Base URL: `/api/gl`

### REST Endpoints

**POST /decisions**
- Evaluate a governed action.
- Request Body: `{use_type, context}`
- Response: `{outcome, justification}`
- Maps to: `decide`

**POST /consent**
- Acquire consent.
- Request Body: `{user_id, purpose, data}`
- Response: `ConsentEnvelope`
- Maps to: Consent acquisition

**GET /attributions/{artifact_id}**
- Resolve attributions.
- Response: Array of `Attribution`
- Maps to: `resolve_attribution`

### gRPC Service

```protobuf
service GovernanceService {
  rpc Decide(DecisionRequest) returns (DecisionResponse);
  rpc AcquireConsent(ConsentRequest) returns (ConsentEnvelope);
  rpc ResolveAttribution(ArtifactId) returns (stream Attribution);
}

message DecisionRequest {
  string use_type = 1;
  string context = 2;
}

message DecisionResponse {
  Outcome outcome = 1;
  string justification = 2;
}

enum Outcome {
  ALLOW = 0;
  DENY = 1;
  ESCALATE = 2;
}
```

### GraphQL Schema

```graphql
type Query {
  attributions(artifactId: ID!): [Attribution!]!
  policies: [Policy!]!
}

type Mutation {
  decide(useType: String!, context: String!): Decision!
  acquireConsent(input: ConsentInput!): ConsentEnvelope!
}

type Decision {
  outcome: Outcome!
  justification: String
}

enum Outcome {
  ALLOW
  DENY
  ESCALATE
}
```

## Intrusion Surface and Boundary Layer (IL) APIs

Base URL: `/api/il`

### REST Endpoints

**POST /detect/style**
- Detect style intrusion.
- Request Body: `{reference_artifact, suspect_artifact}`
- Response: `{intrusion: boolean, score: float}`
- Maps to: `detectStyleIntrusion`

**POST /detect/identity**
- Detect identity intrusion.
- Similar to above.

**GET /incidents**
- List incidents.
- Query Params: `status`, `severity`
- Response: Array of `Incident`

**POST /incidents/{id}/mitigate**
- Mitigate an incident.
- Request Body: `{action}`
- Maps to: `mitigate`

### gRPC Service

```protobuf
service IntrusionService {
  rpc DetectStyle(IntrusionRequest) returns (IntrusionResponse);
  rpc DetectIdentity(IntrusionRequest) returns (IntrusionResponse);
  rpc DetectNarrative(NarrativeRequest) returns (IntrusionResponse);
  rpc ListIncidents(IncidentFilter) returns (stream Incident);
  rpc MitigateIncident(MitigationRequest) returns (MitigationResponse);
}

message IntrusionRequest {
  string reference_artifact = 1;
  string suspect_artifact = 2;
}

message NarrativeRequest {
  string session_id = 1;
  string artifact = 2;
}

message IncidentFilter {
  string status = 1;
  string severity = 2;
}
```

### GraphQL Schema

```graphql
type Query {
  incidents(filter: IncidentFilter): [Incident!]!
  detectStyle(reference: ID!, suspect: ID!): IntrusionResult!
  detectIdentity(reference: ID!, suspect: ID!): IntrusionResult!
}

type Mutation {
  mitigateIncident(id: ID!, action: MitigationAction!): MitigationResponse!
}

type IntrusionResult {
  intrusion: Boolean!
  score: Float
}

enum MitigationAction {
  WARN
  BLOCK
  FLAG
  ESCALATE
}
```

## Cross-Domain Integration Layer (CDIL) APIs

Base URL: `/api/cdil`

### REST Endpoints

**POST /decide**
- Global decision for an operation.
- Request Body: `{operation, context}`
- Response: `{outcome, details}`
- Maps to: `globalDecide`

**GET /consistency**
- Check system consistency.
- Response: `{consistent: boolean, issues: []}`

**POST /profiles/{id}/apply**
- Apply an operational profile.
- Maps to: `applyProfile`

### gRPC Service

```protobuf
service IntegrationService {
  rpc GlobalDecide(OperationRequest) returns (DecisionResponse);
  rpc CheckConsistency(Empty) returns (ConsistencyResponse);
  rpc ApplyProfile(ProfileRequest) returns (ProfileResponse);
}

message OperationRequest {
  string operation = 1;
  string context = 2;
}

message ConsistencyResponse {
  bool consistent = 1;
  repeated string issues = 2;
}
```

### GraphQL Schema

```graphql
type Query {
  consistency: ConsistencyResult!
  profiles: [Profile!]!
}

type Mutation {
  globalDecide(operation: String!, context: String!): GlobalDecision!
  applyProfile(id: ID!): ProfileResponse!
}

type ConsistencyResult {
  consistent: Boolean!
  issues: [String!]
}

type GlobalDecision {
  outcome: Outcome!
  details: String
}
```

## Common Data Types (JSON-LD Compatible)

All APIs support JSON-LD context for semantic interoperability.

Example JSON-LD Context:
```json
{
  "@context": {
    "id": "@id",
    "type": "@type",
    "Event": "capcf:Event",
    "actor": "capcf:actor",
    "timestamp": "capcf:timestamp"
  }
}
```

## Authentication and Authorization

- All endpoints require JWT tokens with user identity.
- GL checks are performed on each request.
- Failed checks log to PL and may trigger IL alerts.

## Interoperability

- gRPC for high-performance inter-service communication.
- REST for external integrations.
- GraphQL for flexible client queries.
- All data serializable to JSON-LD for cross-domain sharing.