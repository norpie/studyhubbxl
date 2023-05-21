DEFINE TABLE location_types SCHEMAFULL;
DEFINE FIELD id ON TABLE location_types TYPE string ASSERT $value != NONE;
DEFINE FIELD path ON TABLE location_types TYPE string ASSERT $value != NONE ;
DEFINE FIELD display_name ON TABLE location_types TYPE string ASSERT $value != NONE;
