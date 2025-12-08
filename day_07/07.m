clear;
input = fileread("input.txt");
lines = strsplit(input, "\n");
maxlen = max(cellfun(@length, lines));

start_loc = [0,0];

grid = zeros(numel(lines), maxlen);

numOfWaysToHit = grid;

EMPTY = 0;
BEAM = 2;
SPLITTER = 1;

for i = 1:numel(lines)
    L = lines{i};
    for j = 1:length(L)
      if L(j) == "S"
        start_loc = [i,j];
        grid(i,j) = BEAM;
        numOfWaysToHit(i,j) = 1;
      endif
      if L(j) == "^"
        grid(i, j) = SPLITTER;
      endif
    endfor
end

splitIdx = [];

for row = 1:size(grid,1)-1
  for col = 1:size(grid,2)

    if grid(row,col) ~= BEAM
      continue;
    end

    ways = numOfWaysToHit(row,col);
    r2 = row + 1;

    % Straight down
    if grid(r2,col) ~= SPLITTER
      grid(r2,col) = BEAM;
      numOfWaysToHit(r2,col) += ways;
      continue;
    end

    % Hit splitter → split
    splitIdx = [splitIdx; r2, col];

    % Right diag
    if col+1 <= size(grid,2)
      grid(r2,col+1) = BEAM;
      numOfWaysToHit(r2,col+1) += ways;
    end

    % Left diag
    if col-1 >= 1
      grid(r2,col-1) = BEAM;
      numOfWaysToHit(r2,col-1) += ways;
    end

  end
end

length(splitIdx)

printf("%.0f\n", sum(transpose(numOfWaysToHit))(end))




