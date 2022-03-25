/** @jsx h */
import {h} from 'preact';

import '../static/bootstrap.min.css';
import '../static/shtats.css';
import UserSummary from "../components/users_summary";
import {generate_commit_data} from "./utils";

export default {
    title: 'Shtats/UserSummary',
    component: UserSummary,
};

const Template = (args) => <UserSummary {...args} />;




export const Default = Template.bind({});
Default.args = {
    data: {
        commits: generate_commit_data(50),
        lines_added: 12345,
        lines_deleted: 3456,
        total_commits: 123,
        index: 1,
        name: "Bobo Jonson",
        gravatar: "44a0e02612a6a98445d56550038e20ae"
    },
};
