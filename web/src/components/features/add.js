import * as React from "react";
import {Create, SimpleForm, TextInput, BooleanInput} from "react-admin";

export const FeatureAdd = (props) => (
  <Create {...props}>
    <SimpleForm>
      <TextInput source="id" />
      <BooleanInput source="value" />
    </SimpleForm>
  </Create>
);
