# Copyright (c) 2023 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
from KFSconfig import KFSconfig
from KFSlog import KFSlog
import flask
import random


@KFSlog.timeit
def main() -> None:
    port: int=6969
    redirect_URL_list: list[str]
    webapp: flask.Flask


    try:
        redirect_URL_list=KFSconfig.load_config("./config/redirect_URL_list.txt", "").split("\n")   # load redirect_URL_list
    except FileNotFoundError as e:
        return
    webapp=flask.Flask(__name__)                                                                    # create webapp


    @webapp.route("/")                                          # add redirect to webapp root
    def redirect():
        return flask.redirect(random.choice(redirect_URL_list)) # redirect to random URL
    
    @webapp.route("/favicon.ico")   # browser tab icon
    def favicon():
        return flask.send_from_directory("../config/", "favicon.ico")
    
    @webapp.route("/ping")  # just send successful response, for testing
    def ping():
        return "Ping has been sucessful.", 200


    webapp.run(port=port)   # start webapp on port
    
    return