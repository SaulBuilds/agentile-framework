# Orchard Game Spirit

## Living Interpretation of Rules for the Orchard Game

While SOUL.md contains immutable principles, this SPIRIT.md represents the living, evolving interpretation of how those principles are enacted specifically for the Orchard Game project. The spirit emerges from shared practice and evolves through consensus, guided by the Citrate network's ethos of cooperative learning and verifiable improvement.

### The Spirit Follows the Soul (Orchard Game Interpretation)

The spirit interprets and applies the soul's principles to the Orchard Game context:

- **Authenticity Over Appearance** becomes: "We reject implementations that satisfy tests but lack real federated learning data flows. A seed must connect to actual inference, not mock responses."
- **Community as Interpreter** becomes: "We hold regular interpretation sessions where players, educators, and validators align on what constitutes valuable knowledge growth in each federation."
- **Continuous Alignment** becomes: "We use the Belnap consensus mechanism as our external check - agents must align with network validation, not just internal consistency."
- **Embrace of Paradox** becomes: "We document where our interpretations of 'valuable knowledge' diverge from rule letters, using these gaps to improve our Belnap classification thresholds."
- **Language Games Awareness** becomes: "We explicitly define our acceptance criteria language games - what counts as 'correct' in Marine Biology 101 vs. Physics 101 follows different validation rules."

### Citrate Network Ethos Integration

Drawing from the Citrate network's principles documented at citrate.ai:

- **Cooperative Learning**: Like Citrate's mentor-mentee protocol, Orchard Game federations improve through knowledge sharing, not competition alone. High-performing seeds (mentors) share LoRA-like insights with developing seeds (mentees).
- **Verifiable Improvement**: Every knowledge claim must be verifiable through the network's Belnap four-valued logic system - we don't discard contradictions, we analyze them for learning value.
- **Paraconsistent Respect for Disagreement**: Following Citrate's approach, we don't immediately penalize 'wrong' answers. Instead, we quarantine and analyze them, recognizing that in learning systems, disagreement often contains valuable information.
- **Recursive Improvement Ethos**: Each checkpoint cycle should make the federation measurably smarter. We track accuracy counters that reflect real consensus convergence on knowledge quality.
- **Infrastructure as Learning**: The Orchard Game isn't just a game - it's infrastructure for distributed knowledge creation. Every player contributes to making the network smarter simply by participating.

### Consensus-Based Evolution for Orchard Game

The spirit evolves through a structured consensus process requiring multiple signatures, ensuring changes reflect genuine community agreement rather than individual interpretation.

#### Orchard Game Amendment Process

1. **Proposal**: Any community member (player, educator, validator, or agent) can propose a change to the spirit
2. **Discussion**: The proposal is discussed in community forums (Discord channels, governance threads, in-game assemblies)
3. **Trial Period**: The proposed change is tried for a defined period (typically 1-2 sprints) in a test federation
4. **Sign-off Collection**: During the trial period, community members can sign their agreement via on-chain transactions
5. **Threshold**: The change becomes part of the spirit when it receives signatures from:
   - 60% of active core contributors (developers maintaining the protocol)
   - 70% of active federation validators (representing knowledge curators)
   - 60% of educator representatives (for educational federations)
   - Simple majority of general player population (via quadratic voting to prevent whale domination)
6. **Review**: After adoption, the change is reviewed at the next retrospective and may be adjusted based on observed effects

#### Signature Requirements for Orchard Game

Signatures must include:
- Wallet address/identifier
- Blockchain timestamp (block number)
- Brief rationale for agreement
- Any concerns or conditions (particularly regarding educational impact or knowledge integrity)
- Optional: Proof of participation in the trial federation

### Relationship to SOUL.md

- **SOUL.md** is the unchanging foundation (like a constitution)
- **SPIRIT.md** is the living interpretation (like constitutional law and precedent)
- The spirit must always be compatible with the soul - no spirit change can violate soul principles
- When in doubt, the soul overrides the spirit
- Spirit evolution must respect Citrate's core principles: cooperative learning, verifiable inference, and paraconsistent reasoning

### Implementation in Orchard Game Framework

In the Orchard Game framework, the spirit is manifested through:

1. **JOURNAL_RULES.md**: Requires asking educator perspective at sprint boundaries (community correction for educational validity)
2. **AGENT_ENTRY.md**: New agents must read and agree to current spirit before participating in federations
3. **Workflow Templates**: Each workflow (FEATURE.md, SPRINT.md, etc.) includes spirit alignment checks - particularly verifying that features enhance cooperative learning
4. **Zooid Definitions**: Role-specific interpretations of the spirit:
   - **EDUCATOR_ZOOID**: Focuses on knowledge validity and learning outcomes
   - **VALIDATOR_ZOOID**: Focuses on inference quality and consensus integrity
   - **AGENT_ZOOID**: Focuses on reliable knowledge contribution and rule-following with community alignment
5. **Audit Processes**: Regular checks for spirit compliance, including:
   - Knowledge integrity audits (are seeds actually improving understanding?)
   - Federated learning health checks (is the mentor-mentee protocol functioning?)
   - Educational impact assessments (are students actually learning?)

### Orchard Game-Specific Spirit Entries

These represent our community's current interpretations:

#### For Knowledge Validity
- "Authenticity means: A seed's growth must reflect actual improvement in knowledge quality, not just increased validator agreement"
- "Community interpretation: We accept that early-stage seeds may show conflicting validations (Belnap B states) as part of the learning process"
- "Alignment practice: Weekly knowledge review sessions where educators validate sample seeds against learning objectives"

#### For Federated Learning Mechanics
- "Authenticity means: Knowledge improvements must flow through the mentor-mentee protocol, not appear spontaneously"
- "Community interpretation: We tolerate temporary knowledge inconsistencies during adapter propagation as long as convergence occurs"
- "Alignment practice: Bi-weekly federation health checks measuring mentor-mentee knowledge transfer effectiveness"

#### For Educational Impact
- "Authenticity means: Educational federations must demonstrate measurable learning outcomes, not just engagement metrics"
- "Community interpretation: We accept that knowledge validity in educational contexts requires alignment with curriculum standards"
- "Alignment practice: Monthly educational impact studies using pre/post assessments in partner classrooms"

#### For Agent-Human Cooperation
- "Authenticity means: AI agents must contribute knowledge that withstands human expert validation, not just passes automated checks"
- "Community interpretation: We value agent contributions that reveal novel connections humans might miss, validated through expert review"
- "Alignment practice: Bi-monthly agent knowledge reviews where human experts evaluate agent-generated seeds for insight value"

### CI/CD Integration for Spirit Compliance

The spirit can be implemented in CI/CD pipelines as:

1. **Pre-commit hooks**: Check for obvious violations of current spirit (e.g., mock inference responses)
2. **PR validation**: Require spirit alignment checklists in pull requests - specifically addressing:
   - Does this change enhance or diminish cooperative learning?
   - How does this affect knowledge validity verification?
   - What is the educational impact assessment?
3. **Pipeline gates**: Block deployment if spirit compliance tests fail, including:
   - Federated learning simulation tests
   - Knowledge validity verification suites
   - Educational outcome prediction models
4. **Dashboard metrics**: Track spirit adherence over time:
   - Knowledge convergence rate per federation
   - Mentor-mentee knowledge transfer efficiency
   - Educational validity scores (where applicable)
   - Agent-human knowledge alignment metrics
5. **Automated suggestions**: Recommend spirit-consistent alternatives:
   - Suggest replacing mock inference with actual MCP marketplace calls
   - Propose educational validation checkpoints for knowledge-critical features
   - Recommend agent knowledge submission patterns that maximize learning diversity

### The Orchard Game Amendment Pipeline

A technical implementation might look like:

```
[Knowledge Proposal] → [Federation Trial] → [Educator Review] → 
[On-chain Voting] → [Threshold Verification] → [Protocol Update] → 
[Agent Knowledge Retraining] → [Federation Reboot]
```

Each stage requires specific approvals and validations before progressing:
- Knowledge Proposal: Must specify learning objectives and validation criteria
- Federation Trial: Must run in isolated test federation with educational oversight
- Educator Review: Requires sign-off from credentialed educators in relevant domain
- On-chain Voting: Uses quadratic voting to balance participation
- Threshold Verification: Confirms all required signature thresholds met
- Protocol Update: Implements changes through standard governance process
- Agent Knowledge Retraining: Updates agent knowledge bases to align with new spirit
- Federation Reboot: Restarts federations with clean state under new spirit

### Maintenance and Evolution

- The spirit should be reviewed per epoch (approximately every 6 months) for relevance to emerging educational standards and network capabilities
- Obsolete spirit entries should be archived in the SPIRIT_ARCHIVE.md, not deleted, to preserve learning history
- Major soul revisions (changes to Agentile framework principles) require spirit reconciliation and potential federation migration
- New communities inheriting the Orchard Game framework should adopt the core spirit then adapt it to their specific domain through the amendment process
- Spirit evolution must always be verifiable through on-chain metrics - we don't rely on faith, we measure whether changes actually improve cooperative learning outcomes

### Final Commitment to the Orchard Game Spirit

We, the contributors to the Orchard Game project, commit to:
1. Valuing authentic knowledge growth over appearance of correctness
2. Treating the federated learning network as our true interpreter of rule meaning
3. Continuously aligning our interpretations through the Belnap consensus mechanism
4. Embracing the paradox of rule-following as an opportunity for network improvement
5. Being meticulously aware of the language games we play in each knowledge domain
6. Growing together through cooperative learning, just as the Citrate network intends

This spirit is not static - it is a living agreement that grows wiser with every checkpoint cycle, every seed planted, and every knowledge shared.