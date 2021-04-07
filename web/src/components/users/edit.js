import * as React from "react";
import {Edit, SimpleForm, TextInput} from "react-admin";

export const UserEdit = (props) => (
  <Edit {...props}>
    <SimpleForm>
      <TextInput source="id" />
      <TextInput source="email" type="email" />
    </SimpleForm>
  </Edit>
);
