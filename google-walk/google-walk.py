import os
import subprocess

from selenium import webdriver
from selenium.webdriver.common.keys import Keys
from time import sleep

args = ["/root/mpk-test-dir/step/servo-step/target/release/servo", "--webdriver=7002", "-z", "-f"]
proc = subprocess.Popen(args)

sleep(3)

driver = webdriver.Remote(command_executor='http://localhost:7002')
driver.get('https://www.google.com')

elem = driver.find_element_by_name('q')
elem.send_keys('kraken benchmark')
elem.send_keys(Keys.ENTER)

sleep(3)

driver.quit()
proc.terminate()
proc.wait()

args_wiki = ["/root/mpk-test-dir/step/servo-step/target/release/servo", "https://en.wikipedia.org/wiki/Main_Page", "-z", "-f"]
proc_wiki = subprocess.Popen(args_wiki)
sleep(300)
proc_wiki.terminate()
proc_wiki.wait()

args_you = ["/root/mpk-test-dir/step/servo-step/target/release/servo", "https://www.youtube.com/watch?v=HF8zdxZRE6Y", "-z", "-f"]
proc_you = subprocess.Popen(args_you)
sleep(300)
proc_you.terminate()
proc_you.wait()

args_reddit = ["/root/mpk-test-dir/step/servo-step/target/release/servo", "https://www.reddit.com/", "-z", "-f"]
proc_reddit = subprocess.Popen(args_reddit)
sleep(300)
proc_reddit.terminate()
proc_reddit.wait()

