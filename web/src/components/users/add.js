import * as React from "react";
import {
  Create,
  SimpleForm,
  SelectInput,
  ReferenceInput,
  TextInput,
  PasswordInput,
} from "react-admin";

export const UserAdd = (props) => (
  <Create {...props}>
    <SimpleForm>
      <TextInput source="email" type="email" />
      <PasswordInput source="password" />
    </SimpleForm>
  </Create>
);
