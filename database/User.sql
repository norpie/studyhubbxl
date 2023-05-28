DEFINE TABLE users SCHEMAFULL;
DEFINE FIELD id ON TABLE users TYPE string ASSERT $value != NONE;
DEFINE FIELD email ON TABLE users TYPE string ASSERT $value != NONE AND is::email($value);
DEFINE FIELD password ON TABLE users TYPE string ASSERT $value != NONE;
DEFINE FIELD salt ON TABLE users TYPE string ASSERT $value != NONE;
