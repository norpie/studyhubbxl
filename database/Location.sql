DEFINE TABLE locations SCHEMAFULL;
DEFINE FIELD id ON TABLE locations TYPE string ASSERT $value != NONE;
DEFINE FIELD name ON TABLE locations TYPE string ASSERT $value != NONE ;
DEFINE FIELD location_type ON TABLE locations TYPE string ASSERT $value != NONE;
DEFINE FIELD attributes ON TABLE locations TYPE string ASSERT $value != NONE;
DEFINE FIELD noise ON TABLE locations TYPE string ASSERT $value != NONE;
DEFINE FIELD address ON TABLE locations TYPE string ASSERT $value != NONE ;
DEFINE FIELD coordinates ON TABLE locations TYPE int ASSERT $value != NONE;
