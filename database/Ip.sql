DEFINE TABLE ip SCHEMAFULL;
DEFINE FIELD user_ip ON TABLE ip TYPE string ASSERT $value != NONE;
DEFINE FIELD window_start ON TABLE ip TYPE string ASSERT $value != NONE;
DEFINE FIELD requests ON TABLE ip TYPE int ASSERT $value != NONE;
