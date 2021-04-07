import * as React from "react";
import {Edit, SimpleForm, TextInput, BooleanInput} from "react-admin";

export const PermissionEdit = (props) => (
  <Edit {...props}>
    <SimpleForm>
      <TextInput source="id" />
      <TextInput source="name" />
    </SimpleForm>
  </Edit>
);
