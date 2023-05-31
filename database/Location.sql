DEFINE TABLE location SCHEMAFULL;
DEFINE FIELD identifier ON TABLE location TYPE string ASSERT $value != NONE;
DEFINE FIELD name ON TABLE location TYPE string ASSERT $value != NONE;
DEFINE FIELD location_type ON TABLE location TYPE string ASSERT $value != NONE;
DEFINE FIELD attributes ON TABLE location TYPE array ASSERT $value != NONE;
DEFINE FIELD noise ON TABLE location TYPE string ASSERT $value != NONE;
DEFINE FIELD address ON TABLE location TYPE string ASSERT $value != NONE;
DEFINE FIELD lat ON TABLE location TYPE number ASSERT $value != NONE;
DEFINE FIELD long ON TABLE location TYPE number ASSERT $value != NONE;
