DEFINE TABLE location_type SCHEMAFULL;
DEFINE FIELD identifier ON TABLE location_type TYPE string ASSERT $value != NONE;
DEFINE FIELD path ON TABLE location_type TYPE string ASSERT $value != NONE ;
DEFINE FIELD display_name ON TABLE location_type TYPE string ASSERT $value != NONE;
