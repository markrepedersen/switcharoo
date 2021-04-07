import * as React from "react";
import {
  Filter,
  TextInput,
  TextField,
  BooleanField,
  EditButton,
  Datagrid,
  List,
} from "react-admin";

const FeatureFilter = (props) => (
  <Filter {...props}>
    <TextInput label="Search" source="q" alwaysOn />
  </Filter>
);

export const FeatureList = (props) => (
  <List filters={<FeatureFilter />} {...props}>
    <Datagrid rowClick="edit">
      <TextField source="id" />
      <BooleanField source="value" />
      <EditButton />
    </Datagrid>
  </List>
);
