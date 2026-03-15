# Collaborative Cryptographic Primitives Over Adversarial Transport — Brief

## Hypothesis

Keel's collaborative coordination primitives (ping/pong/poke, story lifecycle, mission routing) compose naturally over Transit's cryptographically-ordered append-only log to produce a distributed workflow engine where cooperative agents achieve consensus-free coordination with full cryptographic auditability — at fundamentally lower cost than adversarial distributed systems.

The structural properties already present in Keel's primitives (immutable pings, monotonic status transitions, idempotent poke, deterministic routing) are exactly the properties required for convergent replicated state, and Transit's verifiable lineage provides the integrity substrate without contaminating the collaborative hot path.

## Problem Space

Traditional distributed systems assume adversarial participants and pay coordination costs accordingly: Byzantine fault tolerance requires O(n²) message complexity, consensus protocols add latency per round, and identity systems front-load authentication before every action. These costs scale poorly as participant count grows.

Keel operates in a fundamentally different regime — cooperative agents coordinating work through a shared workflow engine. The threat model isn't "participants may lie" but "the transport may be hostile." This distinction changes which cryptographic primitives are load-bearing:

- **Not needed at the collaborative layer:** consensus, BFT, pre-action authentication, non-malleability
- **Needed at the transport layer:** ordering guarantees, tamper evidence, content integrity, non-repudiation
- **Needed at the boundary:** attestation (proving actions happened), not authorization (proving actions are allowed)

Transit (github.com/spoke-sh/transit) provides the transport-layer guarantees through append-only streams with cryptographic segment digests, manifest roots, and lineage checkpoints. The research question is how Keel's collaborative protocols should be designed to maximally exploit this separation.

## Success Criteria

- [ ] Formal mapping between Keel primitives (ping/pong/poke, story lifecycle) and CRDT-like convergence properties — demonstrating that collaborative operations are commutative, idempotent, and monotonic by construction
- [ ] Identity model design that distinguishes environmental identity (LocalSystem), credential identity (Authenticated), and emergent/attested identity (contribution history verified by Transit lineage) — with clear boundaries for where each applies
- [ ] Proof that the staged verification model (checksums on hot path, cryptographic digests at segment boundaries, manifest roots at publication) preserves collaborative throughput while providing adversarial-grade auditability
- [ ] Architectural specification for how Keel's inbox maps to Transit streams, how poke maps to branch-and-merge, and how story lifecycle transitions map to lineage checkpoints
- [ ] Analysis of scaling properties — demonstrating that adding cooperative agents increases system capability (more ping resolvers, richer routing, parallel story execution) rather than increasing coordination cost

## Open Questions

- What is the minimal attestation surface needed at the Transit boundary? Can we avoid per-record signatures entirely and rely on checkpoint-level attestation?
- How should routing rule versioning interact with Transit's lineage DAG? When a poke re-evaluates under updated rules, should the old and new rule versions be explicit in the stream?
- Does the capability-based access model (you can poke because you hold a scoped token) compose cleanly with Transit's checkpoint model, or does it require a separate authorization layer?
- What are the failure modes unique to collaborative-over-adversarial systems? A cooperative agent that crashes mid-poke is different from a Byzantine agent that lies — what recovery semantics does the combination require?
- How does the `ExecutionContext` evolve? Should `Actor` grow an `Attested` variant backed by stream position, or should attestation remain implicit in Transit's lineage?
- Can Merkle Mountain Ranges (from Transit's Stage 2 integrity) provide partial verification of collaborative history — e.g., proving a specific story was accepted without replaying the entire stream?
- What is the game-theoretic equilibrium when cooperative agents have different information (different routing rules, different stream positions)? Does Transit's ordering guarantee make this a non-issue, or are there edge cases?
