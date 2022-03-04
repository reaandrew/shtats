/** @jsx h */
import {h} from 'preact';

import CommitsByHour from '../components/commits_by_hour';
import '../static/bootstrap.min.css';
import '../static/shtats.css';
import {generate_data} from "./utils";

export default {
    title: 'Shtats/Punchcard',
    component: CommitsByHour,
};

const Template = (args) => <CommitsByHour {...args} />;

function generate_punch_data(max_commits) {
    let max_commits_value = max_commits || 50;
    let results = [];
    for (let i = 0; i < 7; i++) {
        for (let j = 0; j < 24; j++) {
            let commits_for_hour = Math.ceil(Math.random() * max_commits_value);
            results.push([i, j, commits_for_hour]);
        }
    }
    return results;
}

export const With30DaysOfData = Template.bind({});
With30DaysOfData.args = {
    data: generate_punch_data(30),
};
