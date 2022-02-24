/** @jsx h */
import { h } from 'preact';

import '../static/bootstrap.min.css';
import '../static/shtats.css';
import LinesAddedDeletedSummary from "../components/lines_added_deleted_summary";
import {generate_data} from "./utils";

export default {
    title: 'Shtats/Lines Added Deleted Summary',
    component: LinesAddedDeletedSummary,
};

const Template = (args) => <LinesAddedDeletedSummary {...args} />;

export const With30DaysOfData = Template.bind({});
With30DaysOfData.args = {
    data: generate_data(30),
};
