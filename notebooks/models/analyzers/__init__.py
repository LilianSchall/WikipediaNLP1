from sklearn.metrics import classification_report, ConfusionMatrixDisplay, confusion_matrix
import matplotlib.pyplot as plt

class ClassifierReport:
    def __init__(self):
        pass

    def report(self, y_test, y_pred, labels):
        print("Classification Report:\n", classification_report(y_test, y_pred))
        matrix = confusion_matrix(y_test, y_pred, labels=labels)
        display = ConfusionMatrixDisplay(confusion_matrix=matrix,
                                         display_labels=labels)
        display.plot()
        plt.show()
