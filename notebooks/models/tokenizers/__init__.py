from nltk.tokenize import word_tokenize, sent_tokenize
import tiktoken

class CustomTokenizer:
    def __init__(self, type="tiktoken"):
        """
        @param type: available types are:
            - tiktoken
            - word
            - sentence
        """
        self.type = type
        match type:
            case "tiktoken":
                self.encoder = tiktoken.encoding_for_model("gpt-4")
                self.tokenizer = self.encoder.decode
            case "word":
                self.tokenizer = word_tokenize
            case "sentence":
                self.tokenizer = sent_tokenize

    def encode(self, content):
        if self.type == "tiktoken":
            return self.encoder.encode(content)
        return self.tokenizer(content)

    def decode(self, content):
        if self.type != "tiktoken":
            raise AssertionError("Decode method can only be used for tiktoken tokenizer")
        return self.encoder.decode(content)



