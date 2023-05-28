DEFINE TABLE attributes SCHEMAFULL;
DEFINE FIELD path ON TABLE attributes TYPE string ASSERT $value != NONE ;
DEFINE FIELD display_name ON TABLE attributes TYPE string ASSERT $value != NONE;
