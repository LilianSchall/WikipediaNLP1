import pandas as pd


def load_wikipedia_dataset(path: str, test_split: float = 0.2, all_infos: bool = False):
    """
    Loads the dataset and returns train and test datasets with given split %
    :return: df_train, df_train
    """

    # Load dataset.csv into a pandas dataframe
    df = pd.read_csv(path, index_col=0)

    # Shuffle dataset
    df = df.sample(frac=1)

    # Split dataset into train and test subsets
    df_train = df.sample(frac=1 - test_split)
    df_test = df.drop(df_train.index)

    # If all infos asked, return the raw datasets
    if all_infos:
        return df_train, df_test

    # Only keep text and category
    df_train.drop(columns=['id', 'url', 'title'], inplace=True)
    df_test.drop(columns=['id', 'url', 'title'], inplace=True)

    return df_train, df_test
