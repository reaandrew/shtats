/** @jsx h */
import {h} from 'preact';

import LinesCollection from '../components/lines_collection';
import '../static/bootstrap.min.css';
import '../static/shtats.css';
import {generate_added_deleted_data} from "./utils";

export default {
    title: 'Shtats/Lines Collection',
    component: LinesCollection,
};

const Template = (args) => <LinesCollection {...args} />;


export const With30DaysOfData = Template.bind({});
With30DaysOfData.args = {
    data: {
        total_lines_by_day: generate_added_deleted_data(30),
        avg_line_stats: {
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
        }
    }
};
