-- ==========================================
-- 1. Users Table
-- ==========================================
CREATE TABLE user_types (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(20) UNIQUE NOT NULL,
    description VARCHAR(255)
);

CREATE TABLE users (
    id VARCHAR(255) NOT NULL PRIMARY KEY gen_random_uuid(),
    name VARCHAR(60) NOT NULL,
    user_type UUID NOT NULL REFERENCES user_types(id), -- 'USER' or 'SYSTEM'
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ==========================================
-- 2. Accounts (Buckets)
-- ==========================================
CREATE TABLE bucket_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(20) UNIQUE NOT NULL,
    description VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE domain_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(20) UNIQUE NOT NULL,
    description VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id VARCHAR(255) NOT NULL REFERENCES users(id),

    domain UUID NOT NULL REFERENCES domain_types(id), -- FIAT, CRYPTO
    currency VARCHAR(20) NOT NULL,
    bucket_type UUID NOT NULL REFERENCES bucket_types(id),

    created_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE (owner_id, domain, currency, bucket_type)
);

-- ==========================================
-- 3. Immutable Ledger (Write Model)
-- ==========================================
CREATE TABLE transaction_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(20) UNIQUE NOT NULL,
    description VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    idempotency_key VARCHAR(255) UNIQUE NOT NULL,
    transaction_type UUID NOT NULL REFERENCES transaction_types(id),
    reference_id VARCHAR(255),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ledger_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seq BIGINT GENERATED ALWAYS AS IDENTITY, -- ordering only, not OCC

    transaction_id UUID NOT NULL REFERENCES transactions(id),
    account_id UUID NOT NULL REFERENCES accounts(id),

    credit DECIMAL(38, 8) CHECK (credit > 0),
    debit  DECIMAL(38, 8) CHECK (debit > 0),

    CONSTRAINT one_side_only CHECK (
        (credit IS NULL) != (debit IS NULL)
    ),

    created_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE (account_id, transaction_id)
);

-- ==========================================
-- 4. Balance Snapshots (Read Model, async updated)
-- ==========================================
CREATE TABLE balance_snapshots (
    owner_id VARCHAR(255) NOT NULL REFERENCES users(id),
    domain VARCHAR(10) NOT NULL,
    currency VARCHAR(20) NOT NULL,

    available_balance DECIMAL(38, 8) NOT NULL DEFAULT 0,
    reserved_balance DECIMAL(38, 8) NOT NULL DEFAULT 0,

    last_available_entry_id UUID REFERENCES ledger_entries(id),
    last_reserved_entry_id  UUID REFERENCES ledger_entries(id),

    updated_at TIMESTAMPTZ DEFAULT NOW(),

    PRIMARY KEY (owner_id, domain, currency)
);

-- ==========================================
-- 5. Indexes
-- ==========================================
CREATE INDEX idx_ledger_account ON ledger_entries(account_id);
CREATE INDEX idx_ledger_account_seq ON ledger_entries(account_id, seq);
CREATE INDEX idx_ledger_transaction ON ledger_entries(transaction_id);
CREATE INDEX idx_snapshots_last_available ON balance_snapshots(last_available_entry_id);
CREATE INDEX idx_snapshots_last_reserved ON balance_snapshots(last_reserved_entry_id);
CREATE INDEX idx_accounts_owner ON accounts(owner_id);