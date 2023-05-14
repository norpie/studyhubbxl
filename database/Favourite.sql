INFO FOR DB;

DEFINE TABLE Favourite SCHEMAFULL;

DEFINE FIELD id ON TABLE user TYPE uuid
ASSERT $value != NONE;
DEFINE FIELD location_id ON TABLE user TYPE string
ASSERT $value != NONE ;
DEFINE FIELD user_id ON TABLE user TYPE string
ASSERT $value != NONE;