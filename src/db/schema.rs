// @generated automatically by Diesel CLI.

diesel::table! {
    certificates (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        created_by_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    questions (id) {
        id -> Int4,
        certificate_id -> Varchar,
        question -> Varchar,
        options -> Nullable<Array<Nullable<Text>>>,
        question_type -> Nullable<Varchar>,
        checkbox -> Nullable<Array<Nullable<Text>>>,
        drop_down -> Nullable<Array<Nullable<Text>>>,
        is_required -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    reimbursements (id) {
        id -> Int4,
        user_id -> Varchar,
        certificate_id -> Varchar,
        amount -> Int4,
        status -> Varchar,
        ifsc -> Varchar,
        account_number -> Varchar,
        certificate_url -> Varchar,
        certificate_file_tye -> Varchar,
        certificate_file_name -> Varchar,
        approved_by_admin -> Bool,
        approved_by_department -> Bool,
        approved_by_reception -> Bool,
        remarks -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        roll -> Varchar,
        password -> Varchar,
        department -> Varchar,
        profile_image -> Nullable<Varchar>,
        academic_year -> Varchar,
    }
}

diesel::joinable!(certificates -> users (created_by_id));
diesel::joinable!(questions -> certificates (certificate_id));
diesel::joinable!(reimbursements -> certificates (certificate_id));
diesel::joinable!(reimbursements -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(certificates, questions, reimbursements, users,);
