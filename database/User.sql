DEFINE TABLE User SCHEMAFULL;

DEFINE FIELD id ON TABLE user TYPE uuid
ASSERT $value != NONE;
DEFINE FIELD email ON TABLE user TYPE string
ASSERT $value != NONE AND is: :email($value);
DEFINE FIELD username ON TABLE user TYPE string
ASSERT $value != NONE;
DEFINE FIELD password ON TABLE user TYPE string
ASSERT $value != NONE AND is: :password($value);
DEFINE FIELD salt ON TABLE user TYPE uuid
ASSERT $value != NONE;