-- Add up migration script here
-- create table for reimbursement
CREATE TABLE Reimbursements (
    id SERIAL NOT NULL PRIMARY KEY UNIQUE,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    certificate_id INTEGER NOT NULL,
    requested_by INTEGER NOT NULL,
    approved_by_admin INTEGER,
    approved_by_hod INTEGER,
    approved_by_accountant INTEGER,
    bank_account_number VARCHAR NOT NULL,
    bank_name VARCHAR NOT NULL,
    bank_ifsc VARCHAR NOT NULL,
    amount INTEGER NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected')),
    -- delete the reimbursement if the certificate is deleted
    FOREIGN KEY (certificate_id) REFERENCES certificates(id) ON DELETE CASCADE,
    FOREIGN KEY (requested_by) REFERENCES users(id) ON DELETE CASCADE,
    -- check if the user id given is admin
    FOREIGN KEY (approved_by_admin) REFERENCES users(id),
    -- check if the user id given is hod
    FOREIGN KEY (approved_by_hod) REFERENCES users(id),
    -- check if the user id given is accountant
    FOREIGN KEY (approved_by_accountant) REFERENCES users(id)
);

CREATE OR REPLACE FUNCTION check_admin() RETURNS TRIGGER AS $$
BEGIN
    -- first check if approved_by_admin is not null and then check if the user is admin
    IF NEW.approved_by_admin IS NOT NULL AND (SELECT role FROM users WHERE id = NEW.approved_by_admin) != 'admin' THEN
        RAISE EXCEPTION 'approved_by_admin must be an admin';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION check_hod() RETURNS TRIGGER AS $$
BEGIN
    -- first check if approved_by_hod is not null and then check if the user is hod
    IF NEW.approved_by_hod IS NOT NULL AND (SELECT role FROM users WHERE id = NEW.approved_by_hod) != 'hod' THEN
        RAISE EXCEPTION 'approved_by_hod must be an hod';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION check_accountant() RETURNS TRIGGER AS $$
BEGIN
    -- first check if approved_by_accountant is not null and then check if the user is accountant
    IF NEW.approved_by_accountant IS NOT NULL AND (SELECT role FROM users WHERE id = NEW.approved_by_accountant) != 'accountant' THEN
        RAISE EXCEPTION 'approved_by_accountant must be an accountant';
    END IF;
    RETURN NEW;
END;

$$ LANGUAGE plpgsql;

-- create trigger for all the check functions created above
CREATE TRIGGER check_admin_trigger BEFORE INSERT OR UPDATE ON reimbursements
    FOR EACH ROW EXECUTE PROCEDURE check_admin();

CREATE TRIGGER check_hod_trigger BEFORE INSERT OR UPDATE ON reimbursements
    FOR EACH ROW EXECUTE PROCEDURE check_hod();

CREATE TRIGGER check_accountant_trigger BEFORE INSERT OR UPDATE ON reimbursements
    FOR EACH ROW EXECUTE PROCEDURE check_accountant();

