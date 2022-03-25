/** @jsx h */
import {h} from 'preact';

import '../static/bootstrap.min.css';
import '../static/shtats.css';
import UsersCollection from "../components/users_collection";
import {generate_commit_data} from "./utils";

export default {
    title: 'Shtats/UserCollection',
    component: UsersCollection,
};

const Template = (args) => <UsersCollection {...args} />;

function generate_users(count){
    let data = [];
    for (let i = 0; i < count; i++) {
        data.push( {
            commits: generate_commit_data(Math.ceil(Math.random()*50)),
            lines_added: Math.ceil(Math.random() * 12345),
            lines_deleted: Math.ceil(Math.random() * 3456),
            total_commits: Math.ceil(Math.random() * 123),
            index: i+1,
            name: `Test User ${i+1}`,
            gravatar: "44a0e02612a6a98445d56550038e20ae"
        })
    }
    return data;
}

export const Default = Template.bind({});
Default.args = {
    data: generate_users(50),
};
