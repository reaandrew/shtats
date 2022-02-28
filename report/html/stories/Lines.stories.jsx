/** @jsx h */
import { h } from 'preact';

import Lines from '../components/lines';
import '../static/bootstrap.min.css';
import '../static/shtats.css';
import {generate_data} from "./utils";

export default {
    title: 'Shtats/Lines Combined',
    component: Lines,
};

const Template = (args) => <Lines {...args} />;


export const With30DaysOfData = Template.bind({});
With30DaysOfData.args = {
    data: generate_data(30),
};
