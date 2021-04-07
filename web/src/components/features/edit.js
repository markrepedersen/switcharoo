import * as React from "react";
import {Edit, SimpleForm, TextInput, BooleanInput} from "react-admin";

export const FeatureEdit = (props) => (
  <Edit {...props}>
    <SimpleForm>
      <TextInput source="id" />
      <BooleanInput source="value" />
    </SimpleForm>
  </Edit>
);
