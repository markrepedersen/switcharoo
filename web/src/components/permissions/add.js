import * as React from "react";
import {Create, SimpleForm, TextInput, BooleanInput} from "react-admin";

export const PermissionAdd = (props) => (
  <Create {...props}>
    <SimpleForm>
      <TextInput source="id" />
      <TextInput source="name" />
    </SimpleForm>
  </Create>
);
