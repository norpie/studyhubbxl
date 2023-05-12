DEFINE TABLE LocationType SCHEMAFULL;

DEFINE FIELD id ON TABLE user TYPE uuid
ASSERT $value != NONE;
DEFINE FIELD path ON TABLE user TYPE string
ASSERT $value != NONE ;
DEFINE FIELD display_name ON TABLE user TYPE string
ASSERT $value != NONE;