DEFINE TABLE favourites SCHEMAFULL;
DEFINE FIELD id ON TABLE favourites TYPE string ASSERT $value != NONE;
DEFINE FIELD location_id ON TABLE favourites TYPE string ASSERT $value != NONE ;
DEFINE FIELD user_id ON TABLE favourites TYPE string ASSERT $value != NONE;
