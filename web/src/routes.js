import React from 'react';
import { Route } from 'react-router-dom';
import SignUp from './components/signup';

export default [
    <Route exact path="/signup" component={SignUp} noLayout />
];
