DEFINE TABLE attributes SCHEMAFULL;
DEFINE FIELD id ON TABLE attributes TYPE string ASSERT $value != NONE;
DEFINE FIELD path ON TABLE attributes TYPE string ASSERT $value != NONE ;
DEFINE FIELD display_name ON TABLE attributes TYPE string ASSERT $value != NONE;
