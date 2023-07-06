-- Add up migration script here
CREATE TABLE PROCESS_REPORT (
    id SERIAL PRIMARY KEY,
    process_kind TEXT,
    records_processed INTEGER GENERATED ALWAYS AS (successfull_records + failed_records) STORED,
    successfull_records INTEGER NOT NULL,
    failed_records INTEGER NOT NULL,
    passed_at TIMESTAMPTZ DEFAULT NOW()
);