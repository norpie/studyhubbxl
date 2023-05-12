DEFINE TABLE Location SCHEMAFULL;

DEFINE FIELD id ON TABLE user TYPE uuid
ASSERT $value != NONE;
DEFINE FIELD name ON TABLE user TYPE string
ASSERT $value != NONE ;
DEFINE FIELD location_type ON TABLE user TYPE string
ASSERT $value != NONE;
DEFINE FIELD attributes ON TABLE user TYPE string
ASSERT $value != NONE 
DEFINE FIELD noise ON TABLE user TYPE string
ASSERT $value != NONE;
DEFINE FIELD address ON TABLE user TYPE string
ASSERT $value != NONE ;
DEFINE FIELD coordinates ON TABLE user TYPE integer
ASSERT $value != NONE;