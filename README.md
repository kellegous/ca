# Cellular Automata

## Getting Started

### Get color themes data

```
curl -o themes.bin https://storage.googleapis.com/fs.kellegous.com/themes-small.bin
```

## 4-color elementary cellular automata

### Render CA with randomly generated rule and theme.
```
cargo run --release -- 4
```
![4 Color CA (rule 17336519114488926371, theme 99545)](images/ca4-17336519114488926371-99545.png)

### Render CA with specified rule and theme.
```
cargo run --release -- 4 --rule=16015190877491773079 --theme=themes.bin:44804
```
![4 Color CA (rule 16015190877491773079, theme 44804)](images/ca4-16015190877491773079-44804.png)

## 1-color elementary cellular automata

### Render CA with randomly generated rule and theme.
```
cargo run --release -- 1
```

![1 Color CA (rule 30, theme 75091)](images/ca1-30-75091.png)

### Render CA with specified rule and theme.
```
cargo run --release -- 1 --rule=90 --theme=themes.bin:12
```

![1 Color CA (rule 90, theme 12)](images/ca1-90-12.png)

### Render CA with specified output file.
```
cargo run --release -- 1 --rule=60 --dest=60.png
```

![1 Color CA (rule 60, theme 90072)](images/ca1-60-90072.png)

## Author
Kelly Norton<br>
kellegous@gmail.com
