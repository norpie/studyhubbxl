DEFINE TABLE session SCHEMAFULL;
DEFINE FIELD identifier ON TABLE session TYPE string ASSERT $value != NONE;
DEFINE FIELD session_id ON TABLE session TYPE string ASSERT $value != NONE;
