/** @jsx h */
import { h } from 'preact';

import '../static/bootstrap.min.css';
import '../static/shtats.css';
import LinesAddedDeletedSummary from "../components/lines_added_deleted_summary";
import {generate_added_deleted_data} from "./utils";
import LinesAddedDeletedSummaryTable from "../components/lines_added_deleted_summary_table";

export default {
    title: 'Shtats/Lines Added Deleted Summary Table',
    component: LinesAddedDeletedSummaryTable,
};

const Template = (args) => <LinesAddedDeletedSummaryTable {...args} />;

export const Default = Template.bind({});
Default.args = {
    data: {
        hour:{
            added: 1,
            deleted: 2,
            churn: 3
        },
        day:{
            added: 1,
            deleted: 2,
            churn: 3
        }  ,
        week: {
            added: 1,
            deleted: 2,
            churn: 3
        },
        month: {
            added: 1,
            deleted: 2,
            churn: 3
        },
        commit: {
            added: 1,
            deleted: 2,
            churn: 3
        }
    },
};
