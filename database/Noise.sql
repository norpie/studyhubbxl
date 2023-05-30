DEFINE TABLE noise SCHEMAFULL;
DEFINE FIELD identifier ON TABLE noise TYPE string ASSERT $value != NONE;
DEFINE FIELD path ON TABLE noise TYPE string ASSERT $value != NONE ;
DEFINE FIELD display_name ON TABLE noise TYPE string ASSERT $value != NONE;
