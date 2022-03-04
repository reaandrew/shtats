/** @jsx h */
import { h } from 'preact';

import LinesCollection from '../components/lines_collection';
import '../static/bootstrap.min.css';
import '../static/shtats.css';
import {generate_data} from "./utils";

export default {
    title: 'Shtats/Lines Collection',
    component: LinesCollection,
};

const Template = (args) => <LinesCollection {...args} />;


export const With30DaysOfData = Template.bind({});
With30DaysOfData.args = {
    data: generate_data(30),
};