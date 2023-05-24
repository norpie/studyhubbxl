DEFINE TABLE noise_levels SCHEMAFULL;
DEFINE FIELD id ON TABLE noise_levels TYPE string ASSERT $value != NONE;
DEFINE FIELD path ON TABLE noise_levels TYPE string ASSERT $value != NONE ;
DEFINE FIELD display_name ON TABLE noise_levels TYPE string ASSERT $value != NONE;
