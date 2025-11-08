# Bounded Contexts

- IdentityBoundedContext: User authentication, authorization, and identity management.
- StashBoundedContext: Manages the core asset vault and the financial ledger of record.
- GovernanceBoundedContext: Manages the binding rules (policies) and the automated operational intents (actions) for each Stash.
- ExecutionBoundedContext: Manages the runtime process, lifecycle, and audit trail of financial movements (transactions).
- AutomationBoundedContext: Contains the specialized logic for scheduling actions and evaluating rules.
- BlockchainBoundedContext: Manages low-level infrastructure and external communication (onchain stash factory and stash operations, network listeners).

## IdentityBoundedContext

User authentication, authorization, and identity management.

### Aggregate Root

User

### Entities

#### User

- int           id
- Uuid          pid
- String        email
- UserStatus    status
- Date          created_at
- Date          last_login_at

#### Session

- int           id
- Uuid          pid
- Uuid          user_id
- String        code
- Date          expires_at

#### UserProfile

- int               id
- Uuid              pid
- Uuid              user_id
- String            display_name
- Optional[String]  avatar_url
- String            wallet_address

### Value Objects

#### UserStatus

- Active
- Suspended
- PendingProfile
- Deleted

### Domain Services

- AuthenticationService
- AuthorizationService
- UserManagementService

### Domain Events

- UserCreated [Outbound]
- UserAuthenticated [Outbound]
- UserLoggedOut [Outbound]

## StashBoundedContext

The Stash Bounded Context manages the core asset holding (the vault) and ledgering functionality, ensuring transactional consistency.

### Aggregate Root ^

Stash

### Entities ^

#### Stash

The central entity and Aggregate Root. It represents the user's asset vault. It is responsible for protecting the consistency of the internal `balances` and `status` and acts as the consistency boundary for asset holdings.

- int                   id
- Uuid                  pid
- Uuid                  user_id
- String                name
- Array[AssetAmount]    balances <!-- list of assets held -->
- StashStatus           status
- Array[String]         tags
- Map                   metadata
- Date                  created_at
- Date                  updated_at

#### LedgerEntry

An immutable financial fact representing a change in the Stash's internal balance. It is created only after an asset movement is confirmed by a completed `ExecutableIntent`.

- int                   id
- Uuid                  pid
- { DEBIT | CREDIT }    type
- Uuid                  stash_id
- Bigint                amount
- Asset                 asset
- Uuid                  intent_id
- Map                   metadata
- Date                  created_at

### Value Objects ^

#### AssetAmount

A value object representing a specific amount of a particular Asset. Ensures all financial calculations are currency-aware and precise.

- Bigint amount
- Asset asset

#### Asset

Defines the unique characteristics of a fungible token or currency. It includes the network context required for transaction routing.

- String name
- String symbol
- String network
- Optional[String] address
- u8 decimals
- u8 display_decimals

#### StashStatus

Enumerates the possible operational states of the Stash vault.

- Active
- Paused
- Closed

### Domain Services ^

- StashService
- StashLedgerService

### Domain Events ^

StepCompleted [Inbound]

BalanceUpdated [Outbound]

StashCreated [Outbound]

StashStatusChanged [Outbound]

## GovernanceBoundedContext

The Governance Bounded Context manages the complex rules (Policy) and user-defined automation (Actions), representing the binding Agreement for the Stash.

### Aggregate Root ^^

StashGovernance

### Entities ^^

#### StashGovernance

The Aggregate Root for the entire governance contract. It encapsulates all Rules (policies) and Actions (scheduled intents) for a single Stash.

- int                       id
- Uuid                      pid
- Uuid                      stash_id
- Array[GovernanceRule]     deposit_rules
- Array[GovernanceRule]     withdrawal_rules
- Array[GovernanceAction]   actions
- Date                      updated_at

#### PenaltyPolicy

Defines the consequence (e.g., fee structure) applied when a specific `GovernanceRule` is violated. Referenced by the Rule.

- int                                       id
- Uuid                                      pid
- String                                    name
- { PercentageForfeit | FixedAmountFine }   type

### Value Objects ^^

#### GovernanceAction

An Entity representing a user's intent (e.g., automated withdrawal) and the conditions (`Triggers`) under which that intent should be initiated.

- int                       id
- Uuid                      pid
- String                    name
- Array[Trigger]            triggers
- Intent                    intent
- { ACTIVE | PAUSED }       status

#### GovernanceRule

Defines the explicit conditions and outcomes of the Stash's policy. It links a state check (`SimplePredicate`) to the permitted action (`IntentType`).

- int               id
- Uuid              pid
- String            name
- SimplePredicate   predicate
- Array[IntentType] permitted_intent_type
- Optional[Uuid]    penalty_policy_id

#### SimplePredicate

The Condition (IF): A simple expression (DSL string) that the `RulesEngine` evaluates against the current `StashState` or the incoming `Intent` parameters.

- String expression

#### Intent

The What: A value object defining the user's high-level financial goal or request. This is the payload carried through the system before execution.

- IntentType    type
- Map           params

#### IntentType

Enumerates the types of financial goals the system recognizes

- DEPOSIT_INTENT
- WITHDRAWAL_INTENT

#### Trigger

The When: Defines the conditions that cause a `GovernanceAction` to initiate, such as a time schedule (`Temporal`) or an external event.

- { Temporal | StateChange | Event | Manual }   type
- Map                                           params <!-- (e.g cron expression, address to monitor) -->

### Domain Services ^^

- GovernancePolicyService
- ActionManagementService

### Domain Events ^^

- GovernanceRuleCreated [Outbound]
- GovernanceRuleUpdated [Outbound]
- ActionCreated [Outbound]
- ActionStatusChanged [Outbound]

## ExecutionBoundedContext

This context manages the execution lifecycle of a financial intent, converting it into a verifiable, auditable workflow of atomic steps.

### Aggregate Root ^^^

ExecutableIntent

### Entities ^^^

#### ExecutableIntent

The Aggregate Root and state machine for a single execution request. It manages the entire lifecycle of the process from initiation to completion/failure, providing a full audit trail.

- int               id
- Uuid              pid
- Uuid              action_ref_id
- Intent            original_intent
- Workflow          workflow
- ExecutionStatus   status
- Date              created_at
- Date              started_at
- Date              completed_at
- Map               metadata

### Value Objects ^^^

Workflow -> Array[ExecutionStep]

Represents the sequential set of atomic `ExecutionSteps` required to fulfill the Intent.

#### ExecutionStep

A single, atomic unit of work guaranteed by the platform's infrastructure (e.g., a single on-chain transfer or API call).
This is a SAGA

- int               step_id
- Operation         operation
- Operation         compensation <!-- a reversal operation in case the major operation fails -->
- Map               params <!-- Arbitrary, step-specific parameters -->
- ExecutionStatus   status
- RetryConfig       retry
- Optional[String]  failure_reason

#### RetryConfig

- int   count
- bool  backoff

#### Operation

Enumerates the types of atomic execution tasks the system can perform.

- TransferOp
- WithdrawOp

#### ExecutionStatus

Enumerates the possible lifecycle states of the `ExecutableIntent`.

- Running
- Waiting
- Completed
- Failed

### Domain Services ^^^

#### ExecutionEngine

Manages the `ExecutableIntent` state machine. It orchestrates the sequence of `ExecutionSteps` and interacts with the `BlockchainBoundedContext` for infrastructure execution.

- start_execution[ExecutableIntent]
- execute_step[ExecutionStep]: emits[StepCompleted] <!-- or it can emit operation based event, e.g TransferOpCompleted-->

### Domain Events ^^^

#### IntentExecutionRequested [Inbound]

The Command fired by the `RulesEngine` that tells the `ExecutionEngine` to create and start a new `ExecutableIntent`.

- Uuid      action_id
- Uuid      stash_id
- Intent    intent

#### StepCompleted [Outbound]

Fired when a single `ExecutionStep` finishes successfully. Used internally by the `ExecutionEngine` to advance the `Workflow` to the next step.

- Uuid          action_id
- Uuid          stash_id
- ExecutionStep execution_step

- ExecutionCompleted [Outbound]
- ExecutionFailed [Outbound]
- CompensationStarted [Outbound]

## AutomationBoundedContext

This supporting context contains the specialized business logic services for scheduling and enforcing rules. It has no Aggregate Roots and operates on data from other contexts.

### Domain Services ^^^^

#### TriggerEngine

The scheduler service. It constantly checks `Temporal` and `Event` triggers defined in the `GovernanceActions`.

- run_schedules[]: emits[ActionReady] <!-- retrieve temporal actions, evaluate schedules, fire event if ready-->
- listen_for_event[]: emits[ActionReady]

#### RulesEngine

The "brain" of the system. It retrieves the `GovernanceRules`, fetches the `StashStateSnapshot` (via the ACL/Projection), evaluates the `SimplePredicate`, and commands the next step.

- evaluate_action[ActionReady,StashStateSnapshot]: emits[IntentExecutionRequested|RuleViolated]

### Domain Events ^^^^

ActionReady [Inbound|Outbound]

Fired by the `TriggerEngine` when a `Trigger` is successfully met. It signals the `RulesEngine` to begin evaluation of the corresponding `Intent`.

- Uuid      action_id
- Uuid      stash_id
- Intent    intent

IntentExecutionRequested [Outbound]

The command to begin execution.

RuleViolated [Outbound]

Fired when an `Intent` fails the `RulesEngine` check. Used to notify the user or trigger a penalty.

- Uuid      stash_id
- Intent    intent
- Uuid      violated_rule_id
  
## BlockchainBoundedContext

Low-level blockchain infrastructure and external communication.

### Aggregate Root ^^^^^

StashVault

### Entities ^^^^^

#### StashVault

- int           id
- Uuid          pid
- Array[Uuid]   assets
- String        contract_address
- String        network
- Date          deployed_at
- Map           deployment_metadata

#### StashAsset

- int           id
- Uuid          pid
- int           onchain_ref_id
- Uuid          vault_id
- String        token
- String        network
- Date          created_at

#### Transaction

- int               id
- Uuid              pid
- String            tx_hash
- String            from_address
- String            to_address
- BigInt            amount
- Asset             asset
- TransactionStatus status
- int               confirmations
- Date              submitted_at
- Date              confirmed_at

#### NetworkListener

- int           id
- Uuid          pid
- String        network_id
- String        contract_address
- Array[String] event_signatures
- Date          created_at

#### NetworkConfig

- int       id
- String    name
- String    symbol
- String    network_id <!-- bsc or bsc-testnet -->
- int       chain_id
- String    native_token_symbol
- String    rpc_url
- String    explorer_url
- int       confirmation_blocks
- int       last_processed_block
- bool      active

### Value Objects ^^^^^

#### TransactionStatus

- PENDING
- SUBMITTED
- CONFIRMING
- CONFIRMED
- FAILED

### Domain Services ^^^^^

- StashFactoryService: Deploys new stash contracts
- TransactionService: Submits and monitors blockchain transactions
- EventListenerService: Monitors blockchain events

### Domain Events ^^^^^

- StashDeployed [Outbound]
- TransactionSubmitted [Outbound]
- TransactionConfirmed [Outbound]
- TransactionFailed [Outbound]
- BlockchainEventDetected [Outbound]

## AuditBoundedContext

Immutable audit trail and compliance logging.

### Entities ^^^^^^

#### AuditLog

- int               id
- Uuid              pid
- Uuid              entity_ref_id
- AuditEventType    event_type
- String            action
- Map               before_state
- Map               after_state
- Optional[String]  ip_address
- Timestamp         timestamp

### Value Objects ^^^^^^

#### AuditEventType

- UserAction
- SystemAction
- SecurityEvent
- DataChange
- AccessAttempt

### Domain Services ^^^^^^

- AuditService

### Domain Events ^^^^^^

- AuditLogCreated [Outbound]
- All domain events [Inbound]
  