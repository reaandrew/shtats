/** @jsx h */
import { h } from 'preact';

import '../static/bootstrap.min.css';
import '../static/shtats.css';
import LinesAddedDeletedSummary from "../components/lines_added_deleted_summary";
import {generate_added_deleted_data} from "./utils";

export default {
    title: 'Shtats/Lines Added Deleted Index',
    component: LinesAddedDeletedSummary,
};

const Template = (args) => <LinesAddedDeletedSummary {...args} />;

export const With30DaysOfData = Template.bind({});
With30DaysOfData.args = {
    data: generate_added_deleted_data(30),
};
