package tempmail

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"strings"
)

var (
	API = "https://www.1secmail.com/api/v1/"
)

type Mail struct {
	Id      int    `json:"id"`
	From    string `json:"from"`
	Subject string `json:"subject"`
	Date    string `json:"date"`
}

type Message struct {
	Id          int      `json:"id"`
	From        string   `json:"from"`
	Subject     string   `json:"subject"`
	Date        string   `json:"date"`
	Attachments []string `json:"attachments"`
	Body        string   `json:"body"`
	TextBody    string   `json:"textBody"`
	HtmlBody    string   `json:"htmlBody"`
}

func Get_mail() (string, error) {
	var emails []string
	err := fetch_json("?action=genRandomMailbox&count=1", &emails)
	if err != nil {
		return "", err
	}

	return emails[0], nil
}

func Read_mail(email string, id int) (Message, error) {
	var mail Message
	auth := strings.Split(email, "@")
	err := fetch_json(fmt.Sprintf("?action=readMessage&login=%s&domain=%s&id=%d", auth[0], auth[1], id), &mail)
	if err != nil {
		return mail, err
	}
	return mail, nil
}

func Check_mail(email string) ([]Mail, error) {
	var mails []Mail
	auth := strings.Split(email, "@")
	err := fetch_json(fmt.Sprintf("?action=getMessages&login=%s&domain=%s", auth[0], auth[1]), &mails)
	if err != nil {
		return mails, err
	}

	return mails, nil
}

func fetch_json(url string, v any) error {
	response, err := http.Get(fmt.Sprintf("%s%s", API, url))
	if err != nil {
		return err
	}

	body, err := ioutil.ReadAll(response.Body)
	if err != nil {
		return err
	}

	json.Unmarshal(body, &v)

	return nil
}
