/** @jsx h */
import { h } from 'preact';

import LinesAddedDeleted from '../components/lines_added_deleted';
import '../static/bootstrap.min.css';
import '../static/shtats.css';
import addDays from '../utils/date';
import {generate_added_deleted_data} from "./utils";

export default {
    title: 'Shtats/Lines Added Deleted',
    component: LinesAddedDeleted,
};

const Template = (args) => <LinesAddedDeleted {...args} />;


export const WithNoData = Template.bind({});
WithNoData.args = {
    data: [],
};

export const With30DaysOfData = Template.bind({});
With30DaysOfData.args = {
    data: generate_added_deleted_data(30),
};

export const With90DaysOfData = Template.bind({});
With90DaysOfData.args = {
    data: generate_added_deleted_data(90),
};

export const With180DaysOfData = Template.bind({});
With180DaysOfData.args = {
    data: generate_added_deleted_data(180),
};

export const With360DaysOfData = Template.bind({});
With360DaysOfData.args = {
    data: generate_added_deleted_data(360),
};

export const With720DaysOfData = Template.bind({});
With720DaysOfData.args = {
    data: generate_added_deleted_data(720),
};

export const With1440DaysOfData = Template.bind({});
With1440DaysOfData.args = {
    data: generate_added_deleted_data(1440),
};

export const With7200DaysOfData = Template.bind({});
With7200DaysOfData.args = {
    data: generate_added_deleted_data(7200),
};